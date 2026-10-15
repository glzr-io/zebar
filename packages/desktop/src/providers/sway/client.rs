use std::{error::Error as StdError, fmt, fmt::Formatter};
use std::future::Future;
use futures::{stream::StreamExt, TryFutureExt};
use futures::stream::Next;
use swayipc_async::{Connection, Error, Event, EventStream, EventType, Fallible};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio_util::sync::CancellationToken;

use crate::providers::sway::SwayOutput;

use tracing::{info, warn, error};
pub struct SwayClient {
  write_con: Connection,
  pub events: EventStream,
}

#[derive(Debug)]
pub struct SwayErrorGroup(Vec<Error>);

impl From<Vec<Error>> for SwayErrorGroup {
  fn from(errs: Vec<Error>) -> Self {
    Self(errs)
  }
}

impl fmt::Display for SwayErrorGroup {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    writeln!(f, "Sway encountered one or more errors. Details:")?;
    for err in &self.0 {
      writeln!(f, "{}", err)?
    };
    Ok(())
  }
}

impl StdError for SwayErrorGroup {}

impl SwayErrorGroup {
  pub fn new(errs: Vec<Error>) -> SwayErrorGroup {
    SwayErrorGroup(errs)
  }
}

impl SwayClient {
  pub async fn new<T: AsRef<[EventType]>>(
    event_types: T,
  ) -> anyhow::Result<SwayClient> {
    Ok(SwayClient {
      write_con: Connection::new().await?,
      events: Connection::new().await?.subscribe(event_types).await?
    })
  }

  pub async fn get_state(&mut self) -> anyhow::Result<SwayOutput> {
    let all_workspaces = self.write_con.get_workspaces().await?;
    let all_outputs = self.write_con.get_outputs().await?;
    let binding_modes = self.write_con.get_binding_modes().await?;
    let binding_state = self.write_con.get_binding_state().await?;
    let active_binding_mode = if !binding_state.is_empty() { Some(binding_state) } else { None };
    Ok(SwayOutput {
      all_workspaces,
      all_outputs,
      binding_modes,
      active_binding_mode,
    })
  }

  pub async fn run_command<T: AsRef<str>>(
    &mut self,
    payload: T,
  ) -> anyhow::Result<()> {
    let errs: Vec<Error> = self.write_con.run_command(payload).await?
      .into_iter()
      .filter_map(Result::err)
      .collect();
    if errs.is_empty() {
      Ok(())
    } else {
      Err(anyhow::Error::from(SwayErrorGroup::new(errs)))
    }
  }
}