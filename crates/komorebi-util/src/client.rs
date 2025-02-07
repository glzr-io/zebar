use std::{
  io::{BufReader, Read},
  time::Duration,
};

use komorebi_client::{Notification, SocketMessage, UnixListener};
use tokio::sync::mpsc;

use crate::{Error, KomorebiOutput};

/// A client that connects to and interacts with Komorebi via IPC on a Unix
/// socket.
pub struct KomorebiClient {
  state_rx: mpsc::Receiver<Result<KomorebiOutput, Error>>,
}

impl KomorebiClient {
  /// Creates a `KomorebiClient` instance.
  ///
  /// The client will immediately begin listening for state changes on the
  /// specified socket.
  ///
  /// Returns an error if the socket connection fails
  pub fn new(socket_name: String) -> crate::Result<Self> {
    let socket = komorebi_client::subscribe(&socket_name)
      .map_err(Error::SocketInitialization)?;

    let (state_tx, state_rx) = mpsc::channel(100);
    Self::listen_socket(socket_name, socket, state_tx);

    Ok(KomorebiClient { state_rx })
  }

  /// Returns the latest state from Komorebi.
  pub async fn output(&self) -> crate::Result<KomorebiOutput> {
    self.state_rx.recv().await.map_err(Error::StreamRead)
  }

  /// Returns the latest state from Komorebi.
  pub async fn output_blocking(&self) -> crate::Result<KomorebiOutput> {
    self.state_rx.recv().map_err(Error::StreamRead)
  }

  /// Listens for socket messages on a separate thread.
  fn listen_socket(
    socket_name: String,
    socket: UnixListener,
    tx: mpsc::Sender<crate::Result<KomorebiOutput>>,
  ) {
    std::thread::spawn(move || {
      for incoming in socket.incoming() {
        match incoming {
          Ok(stream) => {
            let mut buffer = Vec::new();
            let mut reader = BufReader::new(stream);

            // Shutdown signal has been received.
            if matches!(reader.read_to_end(&mut buffer), Ok(0)) {
              tracing::debug!("Komorebi shutdown received.");

              // Attempt to reconnect to Komorebi every 15s.
              while komorebi_client::send_message(
                &SocketMessage::AddSubscriberSocket(socket_name.clone()),
              )
              .is_err()
              {
                std::thread::sleep(Duration::from_secs(15));
              }

              // Successfully reconnected to Komorebi. Continue listening.
              continue;
            }

            // Transform and emit state.
            let result = String::from_utf8(buffer)
              .map_err(Error::InvalidUtf8)
              .and_then(|str| {
                serde_json::from_str::<Notification>(&str)
                  .map_err(Error::NotificationParse)
              })
              .map(|notification| notification.state.into());

            let _ = tx.blocking_send(result);
          }
          Err(_) => {
            let _ = tx.blocking_send(Err(Error::StreamRead));
          }
        }
      }
    });
  }
}
