use anyhow::Context;
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
    let ns_win =
      self.ns_window().context("Failed to get window handle.")? as id;

    unsafe {
      ns_win.setLevel_(
        ((NSMainMenuWindowLevel + 1) as u64)
          .try_into()
          .context("Failed to cast `NSMainMenuWindowLevel`.")?,
      );
    }

    Ok(())
  }
}
