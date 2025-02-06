# systray-util

A library for Windows 10 and 11 for monitoring and interacting with native system tray icons.

## Example usage

```rust
fn main() -> systray_util::Result<()> {
  let mut systray = Systray::new()?;

  // Alternatively use `systray.events().await` to get async events for use
  // with a Tokio runtime.
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

## Technical overview

Uses a "spy" window that intercepts system tray messages directed to `Shell_TrayWnd`. This is done by creating a hidden window with the same class name as `Shell_TrayWnd` and processing `WM_COPYDATA` messages that contain tray icon data (additions, updates, removals).

When a 3rd-party application uses `Shell_NotifyIcon` to add, update, or remove a tray icon, the code inside `Shell32.dll` sends a `WM_COPYDATA` message to the Windows taskbar window. It does this by calling `FindWindow` and looking for a window with the class name of `Shell_TrayWnd`.

### `WM_COPYDATA` messages

- `1`: Appbar messages. Invoked via `SHAppBarMessage`.
    - Affects the position of shell flyouts (e.g. volume and wifi flyouts) and fullscreen behavior of windows.
- `2`: Tray update messages: tray icon additions, updates, and removals. Invoked via `Shell_NotifyIcon`.
    - Necessary for detection of tray icons.
- `3`: Icon position requests. Invoked via `Shell_NotifyIconGetRect`.
    - Affects the position of dropdowns for some tray icons (e.g. OneDrive).
