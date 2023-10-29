use std::time::Duration;

use anyhow::Result;
use sysinfo::{System, SystemExt};
use tauri::{Manager, Runtime};
use tokio::{task, time};

use super::provider_config::ProviderConfig;

pub fn init<R: Runtime>(app: &mut tauri::App<R>) -> tauri::plugin::Result<()> {
  app.manage(ProviderScheduler::new(app.handle()));
  Ok(())
}
pub struct ProviderScheduler {}

impl ProviderScheduler {
  pub fn new<R: Runtime>(app: tauri::AppHandle<R>) -> ProviderScheduler {
    ProviderScheduler {}
  }

  pub async fn register(
    &self,
    options_hash: &str,
    options: ProviderConfig,
    tracked_access: Vec<&str>,
  ) -> Result<()> {
    match options {
      ProviderConfig::Cpu(_) => {
        let forever = task::spawn(async {
          let mut interval = time::interval(Duration::from_millis(5000));

          let mut sys = System::new_all();

          loop {
            interval.tick().await;
            sys.refresh_all();
            println!("=> system:");
            println!("total memory: {} bytes", sys.total_memory());
          }
        });

        forever.await;
        Ok(())
      }
      ProviderConfig::Network(_) => todo!(),
    }
  }
}
