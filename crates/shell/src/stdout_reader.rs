use core::slice::memchr;
use std::io::{BufRead, BufReader};

use crate::{Buffer, Encoding};

/// A pipe reader for stdout/stderr.
pub(crate) struct StdoutReader {
  reader: BufReader<os_pipe::PipeReader>,
  encoding: Encoding,
}

impl StdoutReader {
  /// Creates a new `StdOutReader` instance.
  pub fn new(pipe: os_pipe::PipeReader, encoding: Encoding) -> Self {
    Self {
      reader: BufReader::new(pipe),
      encoding,
    }
  }

  /// Reads the next chunk of data.
  pub fn read_next(&mut self) -> std::io::Result<Option<Buffer>> {
    if self.encoding == Encoding::Raw {
      self.read_raw_chunk()
    } else {
      self.read_line()
    }
  }

  /// Reads a chunk of raw bytes.
  fn read_raw_chunk(&mut self) -> std::io::Result<Option<Buffer>> {
    let chunk = self.reader.fill_buf()?.to_vec();

    if chunk.is_empty() {
      return Ok(None);
    }

    self.reader.consume(chunk.len());
    Ok(Some(Buffer::Raw(chunk)))
  }

  /// Reads until a line ending (\n or \r) is found.
  fn read_line(&mut self) -> std::io::Result<Option<Buffer>> {
    let mut buffer = Vec::new();

    loop {
      let chunk = match self.reader.fill_buf() {
        Ok(chunk) => chunk.to_vec(),
        Err(err) => {
          if err.kind() == std::io::ErrorKind::Interrupted {
            continue;
          } else {
            return Err(err);
          }
        }
      };

      if chunk.is_empty() {
        break;
      }

      match Self::find_delimiter(&chunk) {
        Some(pos) => {
          // Delimiter found - consume up to and including the delimiter.
          // The delimiter is included in the output buffer.
          buffer.extend_from_slice(&chunk[..=pos]);
          self.reader.consume(pos + 1);
          break;
        }
        None => {
          // No delimiter found - consume entire chunk.
          buffer.extend_from_slice(&chunk);
          self.reader.consume(chunk.len());
        }
      }
    }

    if buffer.is_empty() {
      Ok(None)
    } else {
      Ok(Some(self.encoding.decode(buffer)))
    }
  }

  /// Finds the position of a line delimiter (\n or \r) within a buffer.
  fn find_delimiter(buffer: &[u8]) -> Option<usize> {
    // Try to find a newline.
    if let Some(pos) = memchr::memchr(b'\n', buffer) {
      return Some(pos);
    }

    // Try to find a carriage return.
    if let Some(pos) = memchr::memchr(b'\r', buffer) {
      return Some(pos);
    }

    None
  }
}
