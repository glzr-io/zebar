use serde::{Deserialize, Serialize};
use std::{thread, time::Duration};
use libloading::{Library, Symbol};
use widestring::{U16CStr, U16String};
use std::env;
use std::path::PathBuf;
use anyhow::{Result, Context};

use crate::{
    common::SyncInterval,
    providers::{CommonProviderState, Provider, ProviderInputMsg, RuntimeType},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WindowProviderConfig {
    pub refresh_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowOutput {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu: Option<Vec<MenuItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuItem {
    pub name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sub_items: Vec<MenuItem>,
}

// DLL function signatures
type GetActiveWindowTitleFunc = unsafe extern "C" fn() -> *const u16;
type FreeActiveWindowTitleFunc = unsafe extern "C" fn(*const u16);
type GetActiveWindowTopLevelMenuItemsFunc = unsafe extern "C" fn() -> *mut *const u16;
type FreeActiveWindowMenuItemsFunc = unsafe extern "C" fn(*mut *const u16, i32);

pub struct WindowProvider {
    config: WindowProviderConfig,
    common: CommonProviderState,
    lib: Option<Library>,
    dll_error: Option<anyhow::Error>,
    replace_titles: Vec<&'static str>,
    get_title_fn: Option<Symbol<'static, GetActiveWindowTitleFunc>>,
    free_title_fn: Option<Symbol<'static, FreeActiveWindowTitleFunc>>,
    get_menu_items_fn: Option<Symbol<'static, GetActiveWindowTopLevelMenuItemsFunc>>,
    free_menu_items_fn: Option<Symbol<'static, FreeActiveWindowMenuItemsFunc>>,
}

impl WindowProvider {
    pub fn new(config: WindowProviderConfig, common: CommonProviderState) -> Self {
        let dll_path = env::current_exe()
            .ok()
            .and_then(|exe| exe.parent().map(|p| p.join("WindowInfoLibrary.dll")));
    
        let (lib, dll_error) = match dll_path {
            Some(ref path) => match unsafe { Library::new(path) } {
                Ok(lib) => (Some(lib), None),
                Err(e) => (None, Some(anyhow::anyhow!("Failed to load DLL from {}: {}", path.display(), e))),
            },
            None => (None, Some(anyhow::anyhow!("Unable to determine DLL path from current_exe()"))),
        };
    
        let get_title_fn = lib.as_ref()
            .and_then(|lib| unsafe {
                lib.get::<GetActiveWindowTitleFunc>(b"GetActiveWindowTitle\0").ok()
            })
            .map(|s| unsafe {
                std::mem::transmute::<Symbol<GetActiveWindowTitleFunc>, Symbol<'static, GetActiveWindowTitleFunc>>(s)
            });
    
        let free_title_fn = lib.as_ref()
            .and_then(|lib| unsafe {
                lib.get::<FreeActiveWindowTitleFunc>(b"FreeActiveWindowTitle\0").ok()
            })
            .map(|s| unsafe {
                std::mem::transmute::<Symbol<FreeActiveWindowTitleFunc>, Symbol<'static, FreeActiveWindowTitleFunc>>(s)
            });

        let get_menu_items_fn = lib.as_ref()
            .and_then(|lib| unsafe {
                lib.get::<GetActiveWindowTopLevelMenuItemsFunc>(b"GetActiveWindowTopLevelMenuItems\0").ok()
            })
            .map(|s| unsafe {
                std::mem::transmute::<_, Symbol<'static, GetActiveWindowTopLevelMenuItemsFunc>>(s)
            });

        let free_menu_items_fn = lib.as_ref()
            .and_then(|lib| unsafe {
                lib.get::<FreeActiveWindowMenuItemsFunc>(b"FreeActiveWindowMenuItems\0").ok()
            })
            .map(|s| unsafe {
                std::mem::transmute::<_, Symbol<'static, FreeActiveWindowMenuItemsFunc>>(s)
            });
    
        WindowProvider {
            config,
            common,
            lib,
            get_title_fn,
            free_title_fn,
            get_menu_items_fn,
            free_menu_items_fn,
            replace_titles: vec![
                "Zebar - macos/macos",
                "Program Manager",
            ],
            dll_error,
        }
    }    

    fn get_active_window_title(&self) -> Result<String> {
        if let Some(ref err) = self.dll_error {
            return Err(anyhow::anyhow!("DLL load error: {}", err));
        }
    
        let get_title = self.get_title_fn.as_ref()
            .context("Active window title function not loaded")?;
    
        unsafe {
            let title_ptr = get_title();
            if title_ptr.is_null() {
                return Ok(String::new());
            }
    
            let title = U16CStr::from_ptr_str(title_ptr)
                .to_ustring()
                .to_string()
                .context("Failed to convert wide string to Rust String")?;
    
            if let Some(free_title) = &self.free_title_fn {
                free_title(title_ptr);
            }
    
            let normalized = if self.replace_titles.contains(&title.as_str()) {
                "File Explorer".to_string()
            } else {
                let processed_title = title
                .replace(" – ", " - ") // Replace en-dash with space-hyphen-space
                .replace(" — ", " - "); // Replace em-dash with space-hyphen-space
  
                let app_title = processed_title
                    .split(" - ")
                    .last()
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .unwrap_or(&title.as_str());
  
                app_title.to_string()
            };
    
            Ok(normalized)
        }
    }

    fn get_active_window_menu_items(&self) -> Result<Vec<MenuItem>> {
        let get_menu_items = self.get_menu_items_fn.as_ref()
            .context("Menu items function not loaded")?;
    
        let ptr = unsafe { get_menu_items() };
        if ptr.is_null() {
            return Ok(Vec::new());
        }
    
        let mut items = Vec::new();
    
        unsafe {
            let array_ptr = ptr as *const *const u16;
    
            let mut i = 0;
            loop {
                let item_ptr = *array_ptr.add(i);
                if item_ptr.is_null() {
                    break;
                }
            
                let name = U16CStr::from_ptr_str(item_ptr)
                    .to_string()
                    .context("Failed to parse menu string")?;
            
                items.push(MenuItem {
                    name,
                    sub_items: Vec::new(),
                });
            
                i += 1;
            }
            
            if let Some(free_fn) = &self.free_menu_items_fn {
                //free_fn(ptr, i as i32);
            }
        }
    
        Ok(items)
    }

    fn run_interval(&self) -> Result<WindowOutput> {
        let title = match self.get_active_window_title() {
            Ok(t) => t,
            Err(err) => {
                let status_msg = format!("Window title error: {}", err);
                return Ok(WindowOutput {
                    title: status_msg,
                    menu: Some(vec![MenuItem {
                        name: "No menu available".to_string(),
                        sub_items: Vec::new(),
                    }]),
                });
            }
        };

        let menu = match self.get_active_window_menu_items() {
            Ok(items) => Some(items),
            Err(err) => {
                let msg = format!("Window menu error: {}", err);
                Some(vec![MenuItem {
                    name: msg,
                    sub_items: Vec::new(),
                }])
            }
        };
    
        Ok(WindowOutput { title, menu })
    }
    
}

impl Provider for WindowProvider {
    fn runtime_type(&self) -> RuntimeType {
        RuntimeType::Sync
    }

    fn start_sync(&mut self) {
        let mut interval = SyncInterval::new(self.config.refresh_interval);

        loop {
            crossbeam::select! {
                recv(interval.tick()) -> _ => {
                    let output = self.run_interval();
                    self.common.emitter.emit_output(output);
                }
                recv(self.common.input.sync_rx) -> input => {
                    if let Ok(ProviderInputMsg::Stop) = input {
                        break;
                    }
                }
            }
        }
    }
}

impl Drop for WindowProvider {
    fn drop(&mut self) {
        if self.lib.is_some() {
            println!("Unloading WindowInfoLibrary.dll");
        }
    }
}
