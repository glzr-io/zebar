use std::{future::Future, marker::PhantomData, sync::Arc};

use crossbeam::channel as cb_mpsc;
use serde::{Deserialize, Serialize};
use tokio::{
  sync::{mpsc as tk_mpsc, oneshot, Mutex},
  task,
};

use crate::providers::{
  ProviderEmitter,
  ProviderFunctionResponse, ProviderManager, ProviderRef,
  ProviderSender as OldSender,
};

pub trait Provider {
  type Config: 'static + Send;
  type Output;
  type Function: 'static + Send;
  type Response: 'static + Send;
}

pub enum ProviderInputMsg<T> where T : Provider + ?Sized {
  Function(T::Function, oneshot::Sender<anyhow::Result<T::Response>>),
  Stop,
}

pub struct CommonProviderState<T> {
  pub emitter: ProviderEmitter,

  /// Wrapper around the receiver channel for incoming inputs to the
  /// provider.
  pub input_rx: T,

  /// Shared `sysinfo` instance.
  pub sysinfo: Arc<Mutex<sysinfo::System>>,
}

pub trait SyncProvider: Provider {
  fn spawn(
    config: Self::Config,
    common: CommonProviderState<
      cb_mpsc::Receiver<ProviderInputMsg<Self>>,
    >,
  );
}

pub trait AsyncProvider: Provider {
  fn spawn(
    config: Self::Config,
    common: CommonProviderState<
      tk_mpsc::Receiver<ProviderInputMsg<Self>>,
    >,
  ) -> impl Future + Send;
}

fn spawn_sync<T>(
  config_hash: String,
  config: T::Config,
  manager: &ProviderManager,
) -> anyhow::Result<ProviderRef>
where
  T: SyncProvider + 'static,
  ProviderSender: From<cb_mpsc::Sender<ProviderInputMsg<T>>>
{
  let (input_tx, input_rx) = cb_mpsc::bounded(1);
  let common = manager.create_common_state(config_hash.clone(), input_rx);
  let task_handle = task::spawn_blocking(move || {
    T::spawn(config, common);
    tracing::info!("Provider stopped: {}", config_hash);
  });

  Ok(ProviderRef {
    input_tx: OldSender::SelfManaged(input_tx.into()),
    task_handle,
  })
}

fn spawn_async<T>(
  config_hash: String,
  config: T::Config,
  manager: &ProviderManager,
) -> anyhow::Result<ProviderRef>
where
  T: AsyncProvider + 'static,
  ProviderSender: From<tk_mpsc::Sender<ProviderInputMsg<T>>>
{
  let (input_tx, input_rx) = tk_mpsc::channel(1);
  let common = manager.create_common_state(config_hash.clone(), input_rx);
  let task_handle = task::spawn(async move {
    T::spawn(config, common).await;
    tracing::info!("Provider stopped: {}", config_hash);
  });

  Ok(ProviderRef {
    input_tx: OldSender::SelfManaged(input_tx.into()),
    task_handle,
  })
}

macro_rules! impl_from {
    ($(#[$($mac:tt)+])* pub enum $name:ident{$($provider:ident($value_type:ty)),+}) => {
      $(#[$($mac)+])*
      pub enum $name {
        $($provider($value_type),)+
      }
      $(
        impl From<$value_type> for $name {
          fn from(value: $value_type) -> Self {
            Self::$provider(value)
          }
        }
      )+
    };
}

macro_rules! declare_provider_set {
    (sync[$($sync_provider:ident),+] async[$($async_provider:ident),+]) => {
      #[derive(Deserialize, Debug)]
      #[serde(tag = "type", rename_all = "snake_case")]
      pub enum ProviderConfig {
        $($sync_provider(<$sync_provider as Provider>::Config)),+,
        $($async_provider(<$async_provider as Provider>::Config)),+
      }

      impl_from!(
        #[derive(Debug, Clone, PartialEq, Serialize)]
        #[serde(untagged)]
        pub enum ProviderOutput {
          $($sync_provider(<$sync_provider as Provider>::Output)),+,
          $($async_provider(<$async_provider as Provider>::Output)),+
        }
      );

      #[derive(Debug, Clone, Serialize, Deserialize)]
      #[serde(tag = "type", content = "function", rename_all = "snake_case")]
      pub enum ProviderFunction {
        $($sync_provider(<$sync_provider as Provider>::Function)),+,
        $($async_provider(<$async_provider as Provider>::Function)),+
      }

      #[derive(Debug, Clone, Serialize)]
      #[serde(untagged)]
      pub enum ProviderResponse {
        $($sync_provider(<$sync_provider as Provider>::Response)),+,
        $($async_provider(<$async_provider as Provider>::Response)),+
      }

      impl_from!(
        pub enum ProviderSender {
          $($sync_provider(cb_mpsc::Sender<ProviderInputMsg<$sync_provider>>)),+,
          $($async_provider(tk_mpsc::Sender<ProviderInputMsg<$async_provider>>)),+
       }
      );

      pub fn spawn_provider(config_hash: String, config: ProviderConfig, provider_manager: &ProviderManager) -> anyhow::Result<ProviderRef> {
        match config {
          $(
            ProviderConfig::$sync_provider(inner) => spawn_sync::<$sync_provider>(config_hash, inner, provider_manager),
          )+
          $(
            ProviderConfig::$async_provider(inner) => spawn_async::<$async_provider>(config_hash, inner, provider_manager),
          )+
        }
      }

      pub async fn call_function(sender: &ProviderSender, f: ProviderFunction) -> anyhow::Result<ProviderFunctionResponse> {
        let response = match (sender, f) {
          $(
            (ProviderSender::$sync_provider(ref tx), ProviderFunction::$sync_provider(_f)) => {
              let (response_tx, response_rx) = oneshot::channel();
              tx.send(ProviderInputMsg::Function(_f, response_tx))?;
              response_rx.await?.map(|r| ProviderResponse::$sync_provider(r))
            },
          )+
          $(
            (ProviderSender::$async_provider(ref tx), ProviderFunction::$async_provider(_f)) => {
              let (response_tx, response_rx) = oneshot::channel();
              tx.send(ProviderInputMsg::Function(_f, response_tx)).await?;
              response_rx.await?.map(|r| ProviderResponse::$async_provider(r))
            },
          )+
          _ => panic!("Sender type did not match function type"),
        };
        response.map(|r|ProviderFunctionResponse::SelfManaged(r))
      }

      pub async fn stop(sender: &ProviderSender) -> anyhow::Result<()> {
        match (sender) {
          $(
            ProviderSender::$sync_provider(ref tx) => tx.send(ProviderInputMsg::Stop).map_err(anyhow::Error::from),
          )+
          $(
            ProviderSender::$async_provider(ref tx) => tx.send(ProviderInputMsg::Stop).await.map_err(anyhow::Error::from),
          )+
        }
      }
    };
}

pub struct TestSyncProvider {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TestSyncProviderConfig {}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct TestSyncProviderOutput {}

impl Provider for TestSyncProvider {
  type Config = TestSyncProviderConfig;
  type Output = TestSyncProviderOutput;
  type Function = PhantomData<Self>;
  type Response = PhantomData<Self>;
}

impl SyncProvider for TestSyncProvider {
  fn spawn(
    config: <Self as Provider>::Config,
    common: CommonProviderState<
      cb_mpsc::Receiver<ProviderInputMsg<Self>>,
    >,
  ) {
    todo!()
  }
}

pub struct TestAsyncProvider {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TestAsyncProviderConfig();

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct TestAsyncProviderOutput();

impl Provider for TestAsyncProvider {
  type Config = TestAsyncProviderConfig;
  type Output = TestAsyncProviderOutput;
  type Function = PhantomData<Self>;
  type Response = PhantomData<Self>;
}

impl AsyncProvider for TestAsyncProvider {
  async fn spawn(
    config: <Self as Provider>::Config,
    common: CommonProviderState<
      tk_mpsc::Receiver<ProviderInputMsg<Self>>,
    >,
  ) {
    todo!()
  }
}

declare_provider_set!(sync[TestSyncProvider] async[TestAsyncProvider]);
