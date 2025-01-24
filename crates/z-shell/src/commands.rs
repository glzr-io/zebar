use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::process::{CommandEvent, TerminatedPayload};

pub type ProcessId = u32;

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

impl JSCommandEvent {
  pub fn new(event: CommandEvent, encoding: Encoding) -> Self {
    match event {
      CommandEvent::Terminated(payload) => {
        JSCommandEvent::Terminated(payload)
      }
      CommandEvent::Error(error) => JSCommandEvent::Error(error),
      CommandEvent::Stderr(line) => {
        JSCommandEvent::Stderr(encoding.decode(line))
      }
      CommandEvent::Stdout(line) => {
        JSCommandEvent::Stdout(encoding.decode(line))
      }
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Buffer {
  Text(String),
  Raw(Vec<u8>),
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Serialize)]
pub enum Encoding {
  #[serde(rename = "raw")]
  Raw,
  #[serde(rename = "utf-8")]
  Utf8,
  #[serde(rename = "utf-16")]
  Utf16,
  #[serde(rename = "gbk")]
  Gbk,
  #[serde(rename = "gb18030")]
  Gb18030,
  #[serde(rename = "big5")]
  Big5,
  #[serde(rename = "euc-jp")]
  EucJp,
  #[serde(rename = "euc-kr")]
  EucKr,
  #[serde(rename = "iso-2022-jp")]
  Iso2022Jp,
  #[serde(rename = "shift-jis")]
  ShiftJis,
}

impl Encoding {
  pub fn decode(&self, line: Vec<u8>) -> Buffer {
    match self.as_encoding() {
      Some(encoding) => {
        let encoding = encoding.decode_with_bom_removal(&line).0;
        Buffer::Text(encoding.into())
      }
      None => Buffer::Raw(line),
    }
  }

  pub fn as_encoding(&self) -> Option<&'static encoding_rs::Encoding> {
    match self {
      Encoding::Raw => None,
      Encoding::Utf8 => Some(encoding_rs::UTF_8),
      Encoding::Gbk => Some(encoding_rs::GBK),
      Encoding::Gb18030 => Some(encoding_rs::GB18030),
      Encoding::Big5 => Some(encoding_rs::BIG5),
      Encoding::EucJp => Some(encoding_rs::EUC_JP),
      Encoding::Iso2022Jp => Some(encoding_rs::ISO_2022_JP),
      Encoding::ShiftJis => Some(encoding_rs::SHIFT_JIS),
      Encoding::EucKr => Some(encoding_rs::EUC_KR),
      Encoding::Utf16 => Some(encoding_rs::UTF_16LE),
    }
  }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CommandOptions {
  pub cwd: Option<PathBuf>,

  pub env: HashMap<String, String>,

  /// Clear the environment variables of the spawned process.
  pub clear_env: bool,

  /// Character encoding for stdout/stderr.
  pub encoding: Encoding,
}

impl Default for CommandOptions {
  fn default() -> Self {
    Self {
      cwd: None,
      env: HashMap::default(),
      clear_env: true,
      encoding: Encoding::Utf8,
    }
  }
}

#[derive(Debug, Serialize)]
pub struct ChildProcessReturn {
  pub code: Option<i32>,
  pub signal: Option<i32>,
  pub stdout: Buffer,
  pub stderr: Buffer,
}
