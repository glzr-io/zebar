# systray-util · [![Crates.io](https://img.shields.io/crates/v/systray-util.svg)](https://crates.io/crates/systray-util)

A library for Windows 10 and 11 for monitoring and interacting with native system tray icons.

<div align="center">
  <img src="https://github.com/user-attachments/assets/aae2f02e-54a5-4ff0-bb43-364de86c8c80" alt="demo">
  <p><i>Demo of <code>systray-util</code> used for recreating the system tray in Zebar's <code>systray</code> provider.</i></p>
</div>

## Example usage

```rust
use systray_util::{Systray, SystrayEvent, SystrayIconAction};

fn main() -> systray_util::Result<()> {
  let mut systray = Systray::new()?;

  // Alternatively use `systray.events().await` to get async events for use
  // with a tokio runtime.
  while let Some(event) = systray.events_blocking() {
    match event {
      SystrayEvent::IconAdd(icon) => {
        println!("Tray icon added: {:?}", icon);
      }
      SystrayEvent::IconUpdate(icon) => {
        println!("Tray icon updated: {:?}", icon);
      }
      SystrayEvent::IconRemove(id) => {
        println!("Tray icon removed: {:?}", id);
      }
    }
  }

  // Send click action to first icon.
  if let Some(icon) = systray.icons().first() {
    systray.send_action(&icon.stable_id, &SystrayIconAction::LeftClick)?;
  }

  Ok(())
}
```

The `examples/` directory contains sample implementations demonstrating basic usage:

```shell
# Run the synchronous example.
cargo run -p systray-util --example sync

# Run the asynchronous (tokio) example.
cargo run -p systray-util --example async
```

## Technical overview

This library uses a "spy" window to monitor system tray updates. It works by:

1. Creating a hidden window that mimics the Windows taskbar by using the `Shell_TrayWnd` class name.
2. Intercepting `WM_COPYDATA` messages intended for the taskbar that contain system tray icon data.
3. Processing these messages to track icon additions, updates, and removals.
4. Forwarding the original messages to the real `Shell_TrayWnd` to avoid disrupting the native system tray.

When applications use the Windows API [`Shell_NotifyIcon`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shell_notifyiconw) to manage their tray icons, `Shell32.dll` broadcasts `WM_COPYDATA` messages to any window with the `Shell_TrayWnd` class name (found via `FindWindow`). Our spy window receives these same messages, allowing us to monitor all system tray activity.

### `WM_COPYDATA` messages

The following are the three types of messages sent to `Shell_TrayWnd`, identified by their `dwData` value in the `WM_COPYDATA` structure:

- **`1`: Appbar Messages** (triggered by [`SHAppBarMessage`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shappbarmessage))

These messages affect the registration of appbar windows, which in turn affects the behavior of shell flyouts (e.g. volume and wifi flyouts) and fullscreen behavior of windows.

- **`2`: Tray Icon Updates** (triggered by [`Shell_NotifyIcon`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shell_notifyiconw))

These messages contain tray icon data for additions, updates, and removals.

- **`3`: Icon Position Requests** (triggered by [`Shell_NotifyIconGetRect`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shell_notifyicongetrect))

These messages are used to determine tray icon positions. Not very widely used - they affect the context menu position for some applications like OneDrive.
