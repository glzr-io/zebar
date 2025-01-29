use std::sync::OnceLock;

use tokio::sync::mpsc;
use windows::Win32::{
  Foundation::{HWND, LPARAM, LRESULT, WPARAM},
  System::DataExchange::COPYDATASTRUCT,
  UI::WindowsAndMessaging::WM_COPYDATA,
};

use crate::Util;

/// Global instance of sender for platform events.
///
/// For use with window procedure.
static PLATFORM_EVENT_TX: OnceLock<mpsc::UnboundedSender<PlatformEvent>> =
  OnceLock::new();

pub fn run() -> crate::Result<()> {
  let (event_tx, event_rx) = mpsc::unbounded_channel();

  // Add the sender for platform events to global state.
  PLATFORM_EVENT_TX
    .set(event_tx)
    .expect("Platform event sender already set.");

  let window =
    Util::create_message_window("Shell_TrayWnd", Some(window_proc))?;

  Util::run_message_loop();

  Ok(())
}

unsafe extern "system" fn window_proc(
  hwnd: HWND,
  msg: u32,
  wparam: WPARAM,
  lparam: LPARAM,
) -> LRESULT {
  match msg {
    WM_COPYDATA => {
      tracing::info!("Incoming WM_COPYDATA message.");

      let copy_data = match (lparam.0 as *const COPYDATASTRUCT).as_ref() {
        Some(data) => {
          println!("  Got COPYDATASTRUCT");
          data
        }
        None => {
          println!("  Invalid COPYDATASTRUCT pointer");
          return LRESULT(0);
        }
      };

      // Forward to real tray first
      let fwd_result = if let Some(real_tray) = find_real_tray(hwnd) {
        println!("  Forwarding to real tray: {:?}", real_tray);
        SendMessageW(real_tray, WM_COPYDATA, wparam, lparam)
      } else {
        println!("  No real tray found");
        LRESULT(1)
      };

      if copy_data.dwData == 1 && !copy_data.lpData.is_null() {
        println!("  Processing tray data");
        let tray_data =
          std::mem::transmute::<_, &ShellTrayData>(copy_data.lpData);

        println!(
          "  Icon data - hwnd: {:#x}, id: {}, flags: {:#x}",
          tray_data.nid.hwnd_raw, tray_data.nid.uid, tray_data.nid.flags.0
        );

        let window_data_ptr =
          GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut WindowData;
        if !window_data_ptr.is_null() {
          println!("  Got window data pointer");
          let window_data = &mut *window_data_ptr;

          // Update mappings
          window_data.update_icon_mapping(
            tray_data.nid.hwnd_raw,
            tray_data.nid.uid,
          );

          // Get icon data
          if let Some(base64_icon) =
            get_icon_base64(HICON(tray_data.nid.hicon_raw as *mut _))
          {
            println!(
              "  Successfully got icon base64 data, length: {}",
              base64_icon.len()
            );

            // Create the payload
            let payload = json!({
                "header": {
                    "magic": tray_data.dw_magic,
                    "message_type": tray_data.dw_message
                },
                "nid": {
                    "uID": tray_data.nid.uid,
                    "hWnd": format!("{:x}", tray_data.nid.hwnd_raw),
                    "uFlags": tray_data.nid.flags.0,
                    "szTip": String::from_utf16_lossy(&tray_data.nid.sz_tip)
                        .trim_matches(|c| c == '\0' || c == '\u{1}')
                        .to_string(),
                    "icon_data": base64_icon
                }
            });

            println!("  Emitting tray-update event");
            if let Err(e) =
              window_data.app_handle.emit("tray-update", payload)
            {
              println!("  Error emitting event: {:?}", e);
            } else {
              println!("  Event emitted successfully");
            }
          } else {
            println!("  Failed to get icon data");
          }
        } else {
          println!("  Window data pointer is null");
        }
      } else {
        println!("  Invalid or empty tray data");
      }

      fwd_result
    }

    WM_TIMER => {
      let _ = SetWindowPos(
        hwnd,
        HWND_TOPMOST,
        0,
        0,
        0,
        0,
        SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
      );
      LRESULT(0)
    }

    _ => DefWindowProcW(hwnd, msg, wparam, lparam),
  }
}
