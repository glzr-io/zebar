use tauri::{AppHandle, Manager};

/// Fallback port of the GlazeWM IPC server.
const DEFAULT_IPC_PORT: u16 = 6123;

/// Port of this session's GlazeWM IPC server.
///
/// GlazeWM binds to a free port and writes it to a per-session file, so
/// that one instance per logged-in user can run at once. Falls back to the
/// old fixed port, which is what a GlazeWM from before that change listens
/// on.
pub fn ipc_port(app_handle: &AppHandle) -> u16 {
  app_handle
    .path()
    .home_dir()
    .ok()
    .map(|home_dir| {
      home_dir
        .join(".glzr/glazewm")
        .join(format!("ipc-port-{}", session_id()))
    })
    .and_then(|path| std::fs::read_to_string(path).ok())
    .and_then(|port| port.trim().parse().ok())
    .unwrap_or(DEFAULT_IPC_PORT)
}

/// ID of the OS session that Zebar is running in.
///
/// Matches the ID that GlazeWM puts in its port file. Falling back to 0 is
/// harmless: the lookup cannot fail for the current process.
fn session_id() -> u32 {
  #[cfg(target_os = "windows")]
  {
    use windows::Win32::System::{
      RemoteDesktop::ProcessIdToSessionId, Threading::GetCurrentProcessId,
    };

    let mut session_id = 0;

    match unsafe {
      ProcessIdToSessionId(GetCurrentProcessId(), &raw mut session_id)
    } {
      Ok(()) => session_id,
      Err(_) => 0,
    }
  }

  // macOS has a single GUI session at a time.
  #[cfg(not(target_os = "windows"))]
  0
}
