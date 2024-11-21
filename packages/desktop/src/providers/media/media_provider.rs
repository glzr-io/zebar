use crossbeam::channel::{unbounded, Receiver, Sender};
use serde::{Deserialize, Serialize};
use tracing::debug;
use windows::{
  Foundation::{EventRegistrationToken, TypedEventHandler},
  Media::Control::{
    GlobalSystemMediaTransportControlsSession as GsmtcSession,
    GlobalSystemMediaTransportControlsSessionManager as GsmtcManager,
    GlobalSystemMediaTransportControlsSessionMediaProperties as GsmtcMediaProperties,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus as GsmtcPlaybackStatus,
    GlobalSystemMediaTransportControlsSessionTimelineProperties as GsmtcTimelineProperties,
  },
};

use crate::providers::{
  CommonProviderState, MediaFunction, Provider, ProviderFunction,
  ProviderFunctionResponse, ProviderFunctionResult, ProviderInputMsg,
  RuntimeType,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MediaProviderConfig {}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaOutput {
  pub session: Option<MediaSession>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaSession {
  pub title: String,
  pub artist: Option<String>,
  pub album_title: Option<String>,
  pub album_artist: Option<String>,
  pub track_number: u32,
  pub start_time: u64,
  pub end_time: u64,
  pub position: u64,
  pub is_playing: bool,
}

/// Events that can be emitted from media session state changes.
#[derive(Debug)]
enum MediaSessionEvent {
  PlaybackInfoChanged,
  MediaPropertiesChanged,
  TimelinePropertiesChanged,
  SessionChanged,
}

/// Holds event registration tokens for media session callbacks.
///
/// These need to be cleaned up when the session changes.
#[derive(Clone)]
struct EventTokens {
  playback: EventRegistrationToken,
  properties: EventRegistrationToken,
  timeline: EventRegistrationToken,
}

pub struct MediaProvider {
  common: CommonProviderState,
  session: Option<GsmtcSession>,
  event_tokens: Option<EventTokens>,
  event_sender: Sender<MediaSessionEvent>,
  event_receiver: Receiver<MediaSessionEvent>,
}

impl MediaProvider {
  pub fn new(
    _config: MediaProviderConfig,
    common: CommonProviderState,
  ) -> MediaProvider {
    let (event_sender, event_receiver) = unbounded();

    Self {
      common,
      session: None,
      event_tokens: None,
      event_sender,
      event_receiver,
    }
  }

  /// Main entry point that sets up the media session manager and runs the
  /// event loop. This method:
  /// 1. Creates the Windows media session manager
  /// 2. Registers for session change notifications
  /// 3. Sets up the initial session if one exists
  /// 4. Runs the main event loop to handle media state changes
  fn create_session_manager(&mut self) -> anyhow::Result<()> {
    debug!("Creating media session manager.");
    let manager = GsmtcManager::RequestAsync()?.get()?;
    self.register_session_changed_handler(&manager)?;
    self.create_session()?;

    loop {
      crossbeam::select! {
        recv(self.event_receiver) -> event => {
          if let Ok(event) = event {
            debug!("Got media session event: {:?}", event);
            self.handle_event(event)?;
          }
        }
        recv(self.common.input.sync_rx) -> input => {
          match input {
            Ok(ProviderInputMsg::Stop) => {
              break;
            }
            Ok(ProviderInputMsg::Function(
              ProviderFunction::Media(media_function),
              sender,
            )) => {
              if let Some(session) = &self.session {
                let result = match media_function {
                  MediaFunction::Play => {
                    session.TryPlayAsync()?.get()?;
                    Ok(ProviderFunctionResponse::Null)
                  }
                  MediaFunction::Pause => {
                    session.TryPauseAsync()?.get()?;
                    Ok(ProviderFunctionResponse::Null)
                  }
                  MediaFunction::TogglePlayPause => {
                    session.TryTogglePlayPauseAsync()?.get()?;
                    Ok(ProviderFunctionResponse::Null)
                  }
                  MediaFunction::Next => {
                    session.TrySkipNextAsync()?.get()?;
                    Ok(ProviderFunctionResponse::Null)
                  }
                  MediaFunction::Previous => {
                    session.TrySkipPreviousAsync()?.get()?;
                    Ok(ProviderFunctionResponse::Null)
                  }
                };
                sender.send(result).unwrap();
              }
            }
            _ => {}
          }
        }
      }
    }

    Ok(())
  }

  /// Registers a callback with the session manager to detect when the
  /// active media session changes (e.g. when switching between media
  /// players).
  fn register_session_changed_handler(
    &self,
    manager: &GsmtcManager,
  ) -> anyhow::Result<()> {
    let handler = TypedEventHandler::new({
      let sender = self.event_sender.clone();
      move |_, _| {
        sender.send(MediaSessionEvent::SessionChanged).unwrap();
        Ok(())
      }
    });
    manager.CurrentSessionChanged(&handler)?;
    Ok(())
  }

  /// Central event handler that processes all media session events.
  /// Routes events to appropriate update methods based on event type.
  fn handle_event(
    &mut self,
    event: MediaSessionEvent,
  ) -> anyhow::Result<()> {
    match event {
      MediaSessionEvent::SessionChanged => self.create_session()?,
      _ => {
        if let Some(session) = &self.session {
          match event {
            MediaSessionEvent::PlaybackInfoChanged => {
              self.update_playback(session)?
            }
            MediaSessionEvent::MediaPropertiesChanged => {
              self.update_properties(session)?
            }
            MediaSessionEvent::TimelinePropertiesChanged => {
              self.update_timeline(session)?
            }
            _ => {}
          }
        }
      }
    }
    Ok(())
  }

  /// Sets up a new media session when one becomes available.
  /// This includes:
  /// 1. Cleaning up any existing session listeners
  /// 2. Getting the current session from Windows
  /// 3. Setting up new event listeners
  /// 4. Emitting initial state
  fn create_session(&mut self) -> anyhow::Result<()> {
    self.remove_session_listeners();
    let manager = GsmtcManager::RequestAsync()?.get()?;
    let session = manager.GetCurrentSession().ok();

    if let Some(session) = &session {
      self.event_tokens = Some(self.setup_session_listeners(session)?);
      self.emit_full_state(session)?;
    } else {
      self.emit_empty_state();
    }

    self.session = session;
    Ok(())
  }

  /// Creates event listeners for all media session state changes.
  /// Returns tokens needed for cleanup when the session ends.
  fn setup_session_listeners(
    &self,
    session: &GsmtcSession,
  ) -> anyhow::Result<EventTokens> {
    Ok(EventTokens {
      playback: session.PlaybackInfoChanged(&TypedEventHandler::new({
        let sender = self.event_sender.clone();
        move |_, _| {
          sender.send(MediaSessionEvent::PlaybackInfoChanged).unwrap();
          Ok(())
        }
      }))?,
      properties: session.MediaPropertiesChanged(
        &TypedEventHandler::new({
          let sender = self.event_sender.clone();
          move |_, _| {
            sender
              .send(MediaSessionEvent::MediaPropertiesChanged)
              .unwrap();
            Ok(())
          }
        }),
      )?,
      timeline: session.TimelinePropertiesChanged(
        &TypedEventHandler::new({
          let sender = self.event_sender.clone();
          move |_, _| {
            sender
              .send(MediaSessionEvent::TimelinePropertiesChanged)
              .unwrap();
            Ok(())
          }
        }),
      )?,
    })
  }

  /// Cleans up event listeners from the current session.
  fn remove_session_listeners(&mut self) {
    if let (Some(session), Some(tokens)) =
      (&self.session, &self.event_tokens)
    {
      let _ = session.RemovePlaybackInfoChanged(tokens.playback);
      let _ = session.RemoveMediaPropertiesChanged(tokens.properties);
      let _ = session.RemoveTimelinePropertiesChanged(tokens.timeline);
    }
    self.event_tokens = None;
  }

  /// Emits an empty state when no media session is active.
  fn emit_empty_state(&self) {
    self
      .common
      .emitter
      .emit_output(Ok(MediaOutput { session: None }));
  }

  /// Emits a complete state update for all media session properties.
  fn emit_full_state(&self, session: &GsmtcSession) -> anyhow::Result<()> {
    let output = Self::to_media_output(session)?;
    self.common.emitter.emit_output(Ok(output));
    Ok(())
  }

  /// Updates and emits only playback state changes (playing/paused).
  fn update_playback(&self, session: &GsmtcSession) -> anyhow::Result<()> {
    if let Some(mut media_session) =
      Self::to_media_session_output(session)?
    {
      let info = session.GetPlaybackInfo()?;
      media_session.is_playing =
        info.PlaybackStatus()? == GsmtcPlaybackStatus::Playing;
      self.emit_session(media_session);
    }
    Ok(())
  }

  /// Updates and emits only media property changes (title, artist, etc).
  fn update_properties(
    &self,
    session: &GsmtcSession,
  ) -> anyhow::Result<()> {
    if let Some(mut media_session) =
      Self::to_media_session_output(session)?
    {
      let props = session.TryGetMediaPropertiesAsync()?.get()?;
      Self::update_media_properties(&mut media_session, &props)?;
      self.emit_session(media_session);
    }
    Ok(())
  }

  /// Updates and emits only timeline property changes (position/duration).
  fn update_timeline(&self, session: &GsmtcSession) -> anyhow::Result<()> {
    if let Some(mut media_session) =
      Self::to_media_session_output(session)?
    {
      let props = session.GetTimelineProperties()?;
      Self::update_timeline_properties(&mut media_session, &props)?;
      self.emit_session(media_session);
    }
    Ok(())
  }

  /// Helper to emit a media session update through the provider's emitter.
  fn emit_session(&self, session: MediaSession) {
    self.common.emitter.emit_output(Ok(MediaOutput {
      session: Some(session),
    }));
  }

  /// Creates a complete MediaOutput struct from a Windows media session.
  fn to_media_output(
    session: &GsmtcSession,
  ) -> anyhow::Result<MediaOutput> {
    Ok(MediaOutput {
      session: Self::to_media_session_output(session)?,
    })
  }

  /// Creates our MediaSession struct from a Windows media session.
  /// Returns None if the session has no title (indicating invalid/empty
  /// state).
  fn to_media_session_output(
    session: &GsmtcSession,
  ) -> anyhow::Result<Option<MediaSession>> {
    let props = session.TryGetMediaPropertiesAsync()?.get()?;
    let title = props.Title()?.to_string();
    if title.is_empty() {
      return Ok(None);
    }

    // Create base session with title and default values
    let mut media_session = MediaSession {
      title,
      artist: None,
      album_title: None,
      album_artist: None,
      track_number: 0,
      start_time: 0,
      end_time: 0,
      position: 0,
      is_playing: false,
    };

    // Update with current state
    Self::update_media_properties(&mut media_session, &props)?;
    Self::update_timeline_properties(
      &mut media_session,
      &session.GetTimelineProperties()?,
    )?;

    let info = session.GetPlaybackInfo()?;
    media_session.is_playing =
      info.PlaybackStatus()? == GsmtcPlaybackStatus::Playing;

    Ok(Some(media_session))
  }

  /// Updates media metadata properties in a MediaSession struct.
  fn update_media_properties(
    session: &mut MediaSession,
    props: &GsmtcMediaProperties,
  ) -> anyhow::Result<()> {
    let artist = props.Artist()?.to_string();
    let album_title = props.AlbumTitle()?.to_string();
    let album_artist = props.AlbumArtist()?.to_string();

    session.artist = (!artist.is_empty()).then_some(artist);
    session.album_title = (!album_title.is_empty()).then_some(album_title);
    session.album_artist =
      (!album_artist.is_empty()).then_some(album_artist);
    session.track_number = props.TrackNumber()? as u32;

    Ok(())
  }

  /// Updates timeline properties (position/duration) in a MediaSession
  /// struct.
  fn update_timeline_properties(
    session: &mut MediaSession,
    props: &GsmtcTimelineProperties,
  ) -> anyhow::Result<()> {
    session.start_time = props.StartTime()?.Duration as u64 / 10_000_000;
    session.end_time = props.EndTime()?.Duration as u64 / 10_000_000;
    session.position = props.Position()?.Duration as u64 / 10_000_000;
    Ok(())
  }
}

impl Provider for MediaProvider {
  fn runtime_type(&self) -> RuntimeType {
    RuntimeType::Sync
  }

  fn start_sync(&mut self) {
    if let Err(err) = self.create_session_manager() {
      self.common.emitter.emit_output::<MediaOutput>(Err(err));
    }
  }
}
