use tauri::{Runtime, Window};

#[cfg(target_os = "macos")]
use cocoa::{
  appkit::{NSMainMenuWindowLevel, NSWindow},
  base::id,
};

pub trait WindowExt {
  #[cfg(target_os = "macos")]
  fn set_above_menu_bar(&self) -> anyhow::Result<()>;
}

impl<R: Runtime> WindowExt for Window<R> {
  #[cfg(target_os = "macos")]
  fn set_above_menu_bar(&self) -> anyhow::Result<()> {
    use anyhow::Context;

    {
      let ns_win = self
        .ns_window()
        .context("Failed to obtain handle to NSWindow.")?
        as id;

      unsafe {
        ns_win.setLevel_(
          ((NSMainMenuWindowLevel + 1) as u64)
            .try_into()
            .context("Failed to cast NSMainMenuWindowLevel.")?,
        );
      }

      Ok(())
    }
  }
}
