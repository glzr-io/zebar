use std::{ffi::c_void, mem::zeroed, path::PathBuf, sync::OnceLock};

use anyhow::Ok;
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, Sender};
use windows::Win32::{
  Foundation::{HMODULE, HWND, LPARAM, WPARAM},
  Graphics::Gdi::{
    DeleteObject, GetDC, GetDIBits, GetObjectW, BITMAP, BITMAPINFO,
    BITMAPINFOHEADER, DIB_RGB_COLORS,
  },
  UI::{
    Accessibility::{SetWinEventHook, HWINEVENTHOOK},
    WindowsAndMessaging::{
      DestroyIcon, DispatchMessageW, GetClassLongPtrW,
      GetForegroundWindow, GetIconInfo, GetMessageW, GetWindowTextLengthW,
      GetWindowTextW, SendMessageW, TranslateMessage,
      EVENT_SYSTEM_FOREGROUND, GCLP_HICON, GCLP_HICONSM, HICON, ICONINFO,
      ICON_BIG, ICON_SMALL, MSG, WINEVENT_OUTOFCONTEXT, WM_GETICON,
    },
  },
};

use crate::providers::{Provider, ProviderOutput, ProviderResult};

static PROVIDER_TX: OnceLock<mpsc::Sender<ProviderResult>> =
  OnceLock::new();

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FocusedWindowProviderConfig {}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusedWindowOutput {
  pub title: String,
  pub icon: Option<usize>,
}

pub struct FocusedWindowProvider {
  _config: FocusedWindowProviderConfig,
}

// Callback function for SetWinEventHoo::
unsafe extern "system" fn win_event_proc(
  _hook: HWINEVENTHOOK,
  _event: u32,
  _hwnd: HWND,
  _idObject: i32,
  _idChild: i32,
  _dwEventThread: u32,
  _dwmsEventTime: u32,
) {
  if let Some(title) = FocusedWindowProvider::get_foreground_window_title()
  {
    println!("Focused window title: {}", title);
    let icon = FocusedWindowProvider::get_foreground_window_icon(_hwnd);
    let emit_results_tx = PROVIDER_TX.get().clone().unwrap();
    let output = FocusedWindowOutput { title, icon };
    if let Err(err) = emit_results_tx
      .try_send(Ok(ProviderOutput::FocusedWindow(output)).into())
    {
      println!("Error sending result: {:?}", err);
    }
  }
}

#[async_trait]
impl Provider for FocusedWindowProvider {
  async fn run(&self, emit_result_tx: Sender<ProviderResult>) {
    PROVIDER_TX
      .set(emit_result_tx.clone())
      .expect("Error setting provider tx in focused window provider");
    if let Err(err) = self.create_focused_window_hook() {
      emit_result_tx
        .send(Err(err).into())
        .await
        .expect("Error with focused window provider");
    }
  }
}

impl FocusedWindowProvider {
  pub fn new(
    config: FocusedWindowProviderConfig,
  ) -> FocusedWindowProvider {
    FocusedWindowProvider { _config: config }
  }

  fn create_focused_window_hook(&self) -> anyhow::Result<()> {
    unsafe {
      let hook = SetWinEventHook(
        EVENT_SYSTEM_FOREGROUND,
        EVENT_SYSTEM_FOREGROUND,
        HMODULE(std::ptr::null_mut()),
        Some(win_event_proc),
        0,
        0,
        WINEVENT_OUTOFCONTEXT,
      );

      let mut msg = MSG::default();
      while GetMessageW(&mut msg, HWND(std::ptr::null_mut()), 0, 0)
        .as_bool()
      {
        let _ = TranslateMessage(&msg);
        DispatchMessageW(&msg);
      }
      Ok(())
    }
  }

  fn get_foreground_window_title() -> Option<String> {
    unsafe {
      let hwnd = GetForegroundWindow();
      if hwnd.0 == std::ptr::null_mut() {
        return None;
      }

      let length = GetWindowTextLengthW(hwnd);
      if length == 0 {
        return None;
      }

      let mut buffer: Vec<u16> = vec![0; (length + 1) as usize];
      GetWindowTextW(hwnd, &mut buffer);
      let title = String::from_utf16_lossy(&buffer[..length as usize]);
      Some(title)
    }
  }

  fn get_foreground_window_icon(hwnd: HWND) -> Option<usize> {
    unsafe {
      // Attempt to get the large icon first
      let hicon = SendMessageW(
        hwnd,
        WM_GETICON,
        WPARAM(ICON_BIG as usize),
        LPARAM(0),
      );
      let hicon = if hicon.0 == 0 {
        // If no large icon, try to get the small icon
        SendMessageW(
          hwnd,
          WM_GETICON,
          WPARAM(ICON_SMALL as usize),
          LPARAM(0),
        )
      } else {
        hicon
      };

      // Get the icon from the class if none found
      let hicon = if hicon.0 == 0 {
        GetClassLongPtrW(hwnd, GCLP_HICON) as isize
      } else {
        hicon.0
      };

      // Check if there is still no icon
      let hicon = if hicon == 0 {
        GetClassLongPtrW(hwnd, GCLP_HICONSM) as isize
      } else {
        hicon
      };

      if hicon == 0 {
        return None; // No icon available
      }

      let hicon = HICON(hicon as *mut _);
      let mut icon_info = ICONINFO::default();

      if GetIconInfo(hicon, &mut icon_info).is_ok() {
        let hbitmap = icon_info.hbmColor;

        // Create a BITMAP structure to hold the bitmap information
        let mut bmp: BITMAP = zeroed();

        // Get the bitmap information
        let result = GetObjectW(
          hbitmap,
          std::mem::size_of::<BITMAP>() as i32,
          Some(&mut bmp as *mut _ as *mut c_void),
        );
        if result > 0 {
          let width = bmp.bmWidth;
          let height = bmp.bmHeight;

          // Prepare BITMAPINFO for DIB
          let mut bitmap_info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
              biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
              biWidth: width,
              biHeight: -height, // Negative for top-down DIB
              biPlanes: 1,
              biBitCount: 32,
              biCompression: 0, // Use BI_RGB for no compression
              biSizeImage: 0,
              biXPelsPerMeter: 0,
              biYPelsPerMeter: 0,
              biClrUsed: 0,
              biClrImportant: 0,
            },
            bmiColors: Default::default(),
          };

          let mut pixels = vec![0u8; (width * height * 4) as usize]; // Allocate for RGBA

          // Get the DIB bits
          if GetDIBits(
            GetDC(HWND(std::ptr::null_mut())),
            hbitmap,
            0,
            height as u32,
            Some(pixels.as_mut_ptr() as *mut _),
            &mut bitmap_info,
            DIB_RGB_COLORS,
          ) != 0
          {
            // Swap the colors from BGR to RGBA
            for i in 0..(width * height) as usize {
              let b = pixels[i * 4];
              let g = pixels[i * 4 + 1];
              let r = pixels[i * 4 + 2];
              let a = pixels[i * 4 + 3]; // Preserve alpha if present
              pixels[i * 4] = r; // Set red
              pixels[i * 4 + 1] = g; // Set green
              pixels[i * 4 + 2] = b; // Set blue
              pixels[i * 4 + 3] = a; // Set alpha
            }

            let mut output_buf = Vec::new();
            let b64_image = general_purpose::STANDARD
              .encode_slice(&pixels, &mut output_buf)
              .expect("Error encoding focused window icon to base64");

            // Clean up bitmap
            let _ = DeleteObject(hbitmap);

            // Return the path of the saved icon
            return Some(b64_image);
          }
          // Clean up bitmap if DIB bits retrieval fails
          let _ = DeleteObject(hbitmap);
        } else {
          println!("GetObjectW failed: {}", result); // Log failure
        }
      }
      let _ = DestroyIcon(hicon); // Free the icon resources
      None
    }
  }
}
