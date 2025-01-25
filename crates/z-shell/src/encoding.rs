use serde::{Deserialize, Serialize};

use crate::shell::Buffer;

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
    match <&Encoding as TryInto<&'static encoding_rs::Encoding>>::try_into(
      self,
    ) {
      Ok(encoding) => {
        let encoding = encoding.decode_with_bom_removal(&line).0;
        Buffer::Text(encoding.into())
      }
      Err(_) => Buffer::Raw(line),
    }
  }
}

impl TryInto<&'static encoding_rs::Encoding> for &Encoding {
  type Error = ();

  fn try_into(
    self,
  ) -> Result<&'static encoding_rs::Encoding, Self::Error> {
    match self {
      Encoding::Raw => Err(()),
      Encoding::Utf8 => Ok(encoding_rs::UTF_8),
      Encoding::Gbk => Ok(encoding_rs::GBK),
      Encoding::Gb18030 => Ok(encoding_rs::GB18030),
      Encoding::Big5 => Ok(encoding_rs::BIG5),
      Encoding::EucJp => Ok(encoding_rs::EUC_JP),
      Encoding::Iso2022Jp => Ok(encoding_rs::ISO_2022_JP),
      Encoding::ShiftJis => Ok(encoding_rs::SHIFT_JIS),
      Encoding::EucKr => Ok(encoding_rs::EUC_KR),
      Encoding::Utf16 => Ok(encoding_rs::UTF_16LE),
    }
  }
}
