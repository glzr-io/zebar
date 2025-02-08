use std::{
  io::{BufReader, Read, Write},
  time::Duration,
};

use serde_json::json;
use tokio::sync::{mpsc, oneshot};
use uds_windows::{UnixListener, UnixStream};

use crate::{Error, KomorebiOutput};

/// A client that connects to and interacts with Komorebi via IPC on a Unix
/// socket.
pub struct KomorebiClient {
  output_rx: mpsc::Receiver<Result<KomorebiOutput, Error>>,
  shutdown_tx: Option<oneshot::Sender<()>>,
}

impl KomorebiClient {
  /// Creates a `KomorebiClient` instance.
  ///
  /// The client will immediately begin listening for outputs on the
  /// specified socket.
  pub fn new(socket_name: &str) -> crate::Result<Self> {
    let (output_tx, output_rx) = mpsc::channel(100);
    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    Self::subscribe(socket_name.to_string(), output_tx, shutdown_rx);

    Ok(KomorebiClient {
      output_rx,
      shutdown_tx: Some(shutdown_tx),
    })
  }

  /// Returns the latest output from Komorebi socket.
  pub async fn output(&mut self) -> crate::Result<KomorebiOutput> {
    self.output_rx.recv().await.ok_or(Error::SocketRead)?
  }

  /// Returns the latest output from Komorebi socket.
  pub fn output_blocking(&mut self) -> crate::Result<KomorebiOutput> {
    self.output_rx.blocking_recv().ok_or(Error::SocketRead)?
  }

  /// Creates a socket and sends a message to Komorebi to subscribe to it.
  fn create_socket(socket_name: &str) -> crate::Result<UnixListener> {
    let data_dir = dirs::data_local_dir()
      .map(|dir| dir.join("komorebi"))
      .ok_or(Error::DataDir)?;

    let listener_socket = data_dir.join(socket_name);

    if let Err(err) = std::fs::remove_file(&listener_socket) {
      if err.kind() != std::io::ErrorKind::NotFound {
        return Err(Error::SocketInitialization(err));
      }
    }

    let listener = UnixListener::bind(&listener_socket)?;

    let msg_socket = data_dir.join("komorebi.sock");
    let mut msg_stream = UnixStream::connect(msg_socket)?;

    let add_subscriber_msg = serde_json::to_string(&json!({
      "type": "AddSubscriberSocket",
      "content": socket_name.to_string(),
    }))?;

    msg_stream.set_write_timeout(Some(Duration::from_secs(1)))?;
    msg_stream.write_all(add_subscriber_msg.as_bytes())?;

    Ok(listener)
  }

  /// Attempts to create a socket with retry logic.
  fn create_socket_with_retry(
    socket_name: &str,
    shutdown_rx: &mut oneshot::Receiver<()>,
  ) -> Option<UnixListener> {
    loop {
      match Self::create_socket(socket_name) {
        Ok(socket) => return Some(socket),
        Err(err) => {
          tracing::debug!(
            "Failed to connect to Komorebi: {}. Retrying in 15s...",
            err
          );

          std::thread::sleep(Duration::from_secs(15));

          // Check for shutdown signal during retry attempts.
          if shutdown_rx.try_recv().is_ok() {
            return None;
          }
        }
      }
    }
  }

  /// Listens for socket messages on a separate thread.
  fn subscribe(
    socket_name: String,
    output_tx: mpsc::Sender<crate::Result<KomorebiOutput>>,
    mut shutdown_rx: oneshot::Receiver<()>,
  ) {
    std::thread::spawn(move || {
      loop {
        // Attempt to create or recreate socket
        let Some(socket) =
          Self::create_socket_with_retry(&socket_name, &mut shutdown_rx)
        else {
          // Shutdown signal received during connection attempt.
          return;
        };

        for incoming in socket.incoming() {
          // Check whether we should stop listening for incoming messages.
          if shutdown_rx.try_recv().is_ok() {
            return;
          }

          match incoming {
            Ok(stream) => {
              let mut buffer = Vec::new();
              let mut reader = BufReader::new(stream);

              // Shutdown signal has been received.
              if matches!(reader.read_to_end(&mut buffer), Ok(0)) {
                tracing::debug!("Komorebi shutdown received.");
                break;
              }

              // Transform and emit state.
              let result = String::from_utf8(buffer)
                .map_err(Error::InvalidUtf8)
                .and_then(|str| {
                  serde_json::from_str::<KomorebiOutput>(&str)
                    .map_err(Error::OutputParse)
                });

              let _ = output_tx.blocking_send(result);
            }
            Err(_) => {
              let _ = output_tx.blocking_send(Err(Error::SocketRead));
            }
          }
        }
      }
    });
  }

  /// Stops the client and its background listener thread.
  ///
  /// Returns `KomorebiError::AlreadyStopped` if the client was already
  /// stopped.
  pub fn stop(&mut self) -> crate::Result<()> {
    if let Some(shutdown_tx) = self.shutdown_tx.take() {
      // Ignore send errors - if receiver is dropped, thread is already
      // stopped
      let _ = shutdown_tx.send(());
      Ok(())
    } else {
      Err(Error::AlreadyStopped)
    }
  }
}

impl Drop for KomorebiClient {
  fn drop(&mut self) {
    // Attempt to stop the client if it hasn't been stopped already.
    let _ = self.stop();
  }
}
