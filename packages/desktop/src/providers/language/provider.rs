use std::sync::Arc;

use async_trait::async_trait;
use tokio::task::AbortHandle;

use super::{
  language::Language, LanguageProviderConfig, LanguageVariables,
};
use crate::providers::{
  interval_provider::IntervalProvider, variables::ProviderVariables,
};

pub struct LanguageProvider {
  pub config: Arc<LanguageProviderConfig>,
  abort_handle: Option<AbortHandle>,
  state: Arc<Language>,
}

impl LanguageProvider {
  pub fn new(
    config: LanguageProviderConfig,
  ) -> LanguageProvider {
    LanguageProvider {
      config: Arc::new(config),
      abort_handle: None,
      state: Language::new().into(),
    }
  }
}

#[async_trait]
impl IntervalProvider for LanguageProvider {
  type Config = LanguageProviderConfig;

  type State = Language;

  fn config(&self) -> Arc<Self::Config> {
    return self.config.clone();
  }

  fn state(&self) -> Arc<Language> {
    return self.state.clone();
  }

  fn abort_handle(&self) -> &Option<AbortHandle> {
    &self.abort_handle
  }

  fn set_abort_handle(&mut self, abort_handle: AbortHandle) {
    self.abort_handle = Some(abort_handle)
  }

  async fn get_refreshed_variables(
    _: &LanguageProviderConfig,
    lang: &Language,
  ) -> anyhow::Result<ProviderVariables> {
    Ok(ProviderVariables::Language(LanguageVariables {
      language: lang.get_current_language(),
    }))
  }
}
