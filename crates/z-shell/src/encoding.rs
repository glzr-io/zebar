use serde::{Deserialize, Serialize};

use crate::process::Buffer;

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
