use std::{collections::HashMap, path::PathBuf, string::FromUtf8Error};

use encoding_rs::Encoding;
use serde::{Deserialize, Serialize};

use crate::process::{CommandEvent, TerminatedPayload};

pub type ChildId = u32;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "event", content = "payload")]
#[non_exhaustive]
pub enum JSCommandEvent {
  /// Stderr bytes until a newline (\n) or carriage return (\r) is found.
  Stderr(Buffer),
  /// Stdout bytes until a newline (\n) or carriage return (\r) is found.
  Stdout(Buffer),
  /// An error happened waiting for the command to finish or converting
  /// the stdout/stderr bytes to an UTF-8 string.
  Error(String),
  /// Command process terminated.
  Terminated(TerminatedPayload),
}

/// Allowed representation of `Execute` command arguments.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged, deny_unknown_fields)]
#[non_exhaustive]
pub enum ExecuteArgs {
  /// No arguments
  None,

  /// A single string argument
  Single(String),

  /// Multiple string arguments
  List(Vec<String>),
}

fn get_event_buffer(
  line: Vec<u8>,
  encoding: EncodingWrapper,
) -> Result<Buffer, FromUtf8Error> {
  match encoding {
    EncodingWrapper::Text(character_encoding) => {
      match character_encoding {
        Some(encoding) => Ok(Buffer::Text(
          encoding.decode_with_bom_removal(&line).0.into(),
        )),
        None => String::from_utf8(line).map(Buffer::Text),
      }
    }
    EncodingWrapper::Raw => Ok(Buffer::Raw(line)),
  }
}

impl JSCommandEvent {
  pub fn new(event: CommandEvent, encoding: EncodingWrapper) -> Self {
    match event {
      CommandEvent::Terminated(payload) => {
        JSCommandEvent::Terminated(payload)
      }
      CommandEvent::Error(error) => JSCommandEvent::Error(error),
      CommandEvent::Stderr(line) => get_event_buffer(line, encoding)
        .map(JSCommandEvent::Stderr)
        .unwrap_or_else(|e| JSCommandEvent::Error(e.to_string())),
      CommandEvent::Stdout(line) => get_event_buffer(line, encoding)
        .map(JSCommandEvent::Stdout)
        .unwrap_or_else(|e| JSCommandEvent::Error(e.to_string())),
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Buffer {
  Text(String),
  Raw(Vec<u8>),
}

#[derive(Debug, Copy, Clone)]
pub enum EncodingWrapper {
  Raw,
  Text(Option<&'static Encoding>),
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandOptions {
  pub cwd: Option<PathBuf>,

  /// By default we don't add any env variables to the spawned process
  /// but the env is an `Option` so when it's `None` we clear the env.
  #[serde(default = "default_env")]
  pub env: Option<HashMap<String, String>>,

  /// Character encoding for stdout/stderr.
  pub encoding: Option<String>,
}

#[allow(clippy::unnecessary_wraps)]
fn default_env() -> Option<HashMap<String, String>> {
  Some(HashMap::default())
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Output {
  String(String),
  Raw(Vec<u8>),
}

#[derive(Debug, Serialize)]
pub struct ChildProcessReturn {
  pub code: Option<i32>,
  pub signal: Option<i32>,
  pub stdout: Output,
  pub stderr: Output,
}
