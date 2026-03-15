use anyhow::Context;
use cocoa::{
  appkit::{NSMainMenuWindowLevel, NSWindow},
  base::id,
};
use tauri::{Runtime, Window};

pub enum CustomWindowLevel {
  AboveMenuBar,
  // TODO: Use this for bottom-most windows.
  Backstop,
}

pub trait WindowExtMacOs {
  fn set_level(&self, level: CustomWindowLevel) -> anyhow::Result<()>;
}

impl<R: Runtime> WindowExtMacOs for Window<R> {
  fn set_level(&self, level: CustomWindowLevel) -> anyhow::Result<()> {
    let ns_win =
      self.ns_window().context("Failed to get window handle.")? as id;

    let level = match level {
      CustomWindowLevel::AboveMenuBar => NSMainMenuWindowLevel as i64 + 1,
      CustomWindowLevel::Backstop => -20,
    };

    unsafe {
      ns_win.setLevel_(level);
    }

    Ok(())
  }
}
