use anyhow::Result;
use tauri::{Manager, Runtime};

use super::provider_config::CpuProviderConfig;

pub fn init<R: Runtime>(app: &mut tauri::App<R>) -> tauri::plugin::Result<()> {
  let handle = app.handle();

  app.manage(ProviderScheduler::new());

  Ok(())
}

pub struct ProviderScheduler {}

impl ProviderScheduler {
  pub fn new() -> Result<ProviderScheduler> {
    Ok(ProviderScheduler {})
  }

  pub fn register(
    &self,
    options_hash: &str,
    options: CpuProviderConfig,
    tracked_access: Vec<&str>,
  ) -> Result<()> {
    todo!()
    // match
  }
}
