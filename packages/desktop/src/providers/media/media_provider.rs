use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::{sync::mpsc::Sender, time};
use windows::{
  Foundation::{EventRegistrationToken, TypedEventHandler},
  Media::Control::{
    GlobalSystemMediaTransportControlsSession,
    GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionMediaProperties,
  },
};

use crate::{
  impl_interval_provider,
  providers::{Provider, ProviderOutput, ProviderResult},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MediaProviderConfig {
  pub refresh_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaOutput {
  pub title: String,
  pub artist: String,
  pub album: String,
  pub album_artist: String,
  //   pub duration: u64,
  //   pub position: u64,
  //   pub is_playing: bool,
}

pub struct MediaProvider {
  _config: MediaProviderConfig,
  session_manager: GlobalSystemMediaTransportControlsSessionManager,
}

impl MediaProvider {
  pub fn new(config: MediaProviderConfig) -> MediaProvider {
    let session_manager =
      GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
        .expect("msg")
        .get()
        .expect("msg");
    println!("Session manager obtained.");

    // Initial check for the current session
    if let Ok(current_session) = session_manager.GetCurrentSession() {
      println!("Initial current session obtained.");
      if let Err(e) =
        MediaProvider::print_current_media_info(&current_session)
      {
        eprintln!("Failed to get media properties: {:?}", e);
      }

      // Attach an event listener to the current session for media property
      // changes
      let media_properties_changed_handler = TypedEventHandler::new(
        move |session: &Option<
          GlobalSystemMediaTransportControlsSession,
        >,
              _| {
          println!("Media properties changed event triggered.");
          if let Some(session) = session {
            if let Err(e) =
              MediaProvider::print_current_media_info(session)
            {
              eprintln!("Failed to get media properties: {:?}", e);
            }
          } else {
            println!("No session available on media properties change.");
          }
          Ok(())
        },
      );

      if let Err(e) = current_session
        .MediaPropertiesChanged(&media_properties_changed_handler)
      {
        eprintln!(
          "Failed to attach media properties changed handler: {:?}",
          e
        );
      } else {
        println!("Media properties changed handler attached.");
      }
    } else {
      println!("No initial current session available.");
    }

    let session_changed_handler = TypedEventHandler::new(
      |session_manager: &Option<
        GlobalSystemMediaTransportControlsSessionManager,
      >,
       _| {
        println!("Session changed event triggered.");
        if let Some(session_manager) = session_manager {
          if let Ok(current_session) = session_manager.GetCurrentSession()
          {
            println!("Current session obtained.");
            if let Err(e) =
              MediaProvider::print_current_media_info(&current_session)
            {
              eprintln!("Failed to get media properties: {:?}", e);
            }

            // Attach an event listener to the current session for media
            // property changes
            let media_properties_changed_handler = TypedEventHandler::new(
              move |session: &Option<
                GlobalSystemMediaTransportControlsSession,
              >,
                    _| {
                println!("Media properties changed event triggered.");
                if let Some(session) = session {
                  if let Err(e) =
                    MediaProvider::print_current_media_info(session)
                  {
                    eprintln!("Failed to get media properties: {:?}", e);
                  }
                } else {
                  println!(
                    "No session available on media properties change."
                  );
                }
                Ok(())
              },
            );

            if let Err(e) = current_session
              .MediaPropertiesChanged(&media_properties_changed_handler)
            {
              eprintln!(
                "Failed to attach media properties changed handler: {:?}",
                e
              );
            } else {
              println!("Media properties changed handler attached.");
            }
          } else {
            println!("No current session available.");
          }
        } else {
          println!("No session manager available.");
        }
        Ok(())
      },
    );

    if let Err(e) =
      session_manager.CurrentSessionChanged(&session_changed_handler)
    {
      eprintln!("Failed to attach session changed handler: {:?}", e);
    } else {
      println!("Event handler for session changes attached.");
    }
    MediaProvider {
      _config: config,
      session_manager,
    }
  }

  fn print_current_media_info(
    session: &GlobalSystemMediaTransportControlsSession,
  ) -> Result<()> {
    let media_properties = session.TryGetMediaPropertiesAsync()?.get()?;
    println!("Title: {}", media_properties.Title()?);
    println!("Artist: {}", media_properties.Artist()?);
    println!("Album: {}", media_properties.AlbumTitle()?);
    println!("Album Artist: {}", media_properties.AlbumArtist()?);
    Ok(())
  }
}

#[async_trait]
impl Provider for MediaProvider {
  async fn run(&self, emit_result_tx: Sender<ProviderResult>) {
    // if let Err(err) = self.bind_media_events() {
    // emit_result_tx.send(Err(err).into()).await;
    // }
  }
}

// impl_interval_provider!(MediaProvider, true);
