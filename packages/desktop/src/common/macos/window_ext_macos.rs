use cocoa::{
  appkit::{NSMainMenuWindowLevel, NSWindow},
  base::id,
};
use tauri::{Runtime, Window};

pub trait WindowExtMacOs {
  fn set_above_menu_bar(&self) -> anyhow::Result<()>;
}

impl<R: Runtime> WindowExtMacOs for Window<R> {
  fn set_above_menu_bar(&self) -> anyhow::Result<()> {
    let window = self.clone();

    // AppKit aborts the process when `setLevel:` is called off the main
    // thread, which is the case for widgets started via IPC. Off the main
    // thread, Tauri queues this task on the same event loop as later
    // window calls (e.g. `set_position`), so they still run after it.
    self.run_on_main_thread(move || {
      let ns_win = match window.ns_window() {
        Ok(ns_win) => ns_win as id,
        Err(err) => {
          tracing::warn!("Failed to get window handle: {}", err);
          return;
        }
      };

      unsafe {
        ns_win.setLevel_(NSMainMenuWindowLevel as i64 + 1);
      }
    })?;

    Ok(())
  }
}
