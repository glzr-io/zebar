use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio::{sync::mpsc::Sender, sync::Mutex, task::AbortHandle};

use crate::providers::{
  manager::ProviderOutput, provider::Provider,
  variables::ProviderVariables,
};

use super::{KomorebiProviderConfig, KomorebiVariables};

pub struct KomorebiProvider {
  pub config: Arc<KomorebiProviderConfig>,
  abort_handle: Option<AbortHandle>,
  // sysinfo: Arc<Mutex<System>>,
}

impl KomorebiProvider {
  pub fn new(
    config: KomorebiProviderConfig,
    // sysinfo: Arc<Mutex<System>>,
  ) -> KomorebiProvider {
    KomorebiProvider {
      config: Arc::new(config),
      abort_handle: None,
      // sysinfo,
    }
  }
}

#[async_trait]
impl Provider for KomorebiProvider {
  async fn on_start(
    &mut self,
    config_hash: String,
    emit_output_tx: Sender<ProviderOutput>,
  ) {
    // Ok(ProviderVariables::Komorebi(KomorebiVariables {
    //   workspaces: 0,
    // }))
  }
  async fn on_refresh(
    &mut self,
    config_hash: String,
    emit_output_tx: Sender<ProviderOutput>,
  ) {
    //
  }

  async fn on_stop(&mut self) {
    //
  }
}
