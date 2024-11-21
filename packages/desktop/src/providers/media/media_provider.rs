use std::collections::HashMap;

use anyhow::Context;
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
  ProviderFunctionResponse, ProviderInputMsg, RuntimeType,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MediaProviderConfig {}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaOutput {
  pub current_session: Option<MediaSession>,
  pub all_sessions: Vec<MediaSession>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaSession {
  pub session_id: String,
  pub title: String,
  pub artist: Option<String>,
  pub album_title: Option<String>,
  pub album_artist: Option<String>,
  pub track_number: u32,
  pub start_time: u64,
  pub end_time: u64,
  pub position: u64,
  pub is_playing: bool,
  pub is_current_session: bool,
}

/// Events that can be emitted from media session state changes.
#[derive(Debug)]
enum MediaSessionEvent {
  SessionListChanged,
  CurrentSessionChanged(Option<String>),
  PlaybackInfoChanged(String),
  MediaPropertiesChanged(String),
  TimelinePropertiesChanged(String),
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
  current_session_id: Option<String>,
  sessions: HashMap<String, (GsmtcSession, EventTokens)>,
  session_outputs: HashMap<String, MediaSession>,
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
      current_session_id: None,
      sessions: HashMap::new(),
      session_outputs: HashMap::new(),
      event_tokens: None,
      event_sender,
      event_receiver,
    }
  }

  /// Main entry point that sets up the media session manager and runs the
  /// event loop.
  fn create_session_manager(&mut self) -> anyhow::Result<()> {
    debug!("Getting media session manager.");
    let manager = GsmtcManager::RequestAsync()?.get()?;

    self.register_session_changed_handler(&manager)?;
    self.create_session(&manager)?;

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
              let function_res = self.handle_function(media_function).map_err(|err| err.to_string());
              sender.send(function_res).unwrap();
            }
            _ => {}
          }
        }
      }
    }

    Ok(())
  }

  /// Registers callbacks with the session manager.
  ///
  /// - `CurrentSessionChanged`: for when the active media session changes
  ///   (e.g. when switching between media players).
  /// - `SessionListChanged`: for when the list of available media sessions
  ///   changes (e.g. when a media player is opened or closed).
  fn register_session_changed_handler(
    &self,
    manager: &GsmtcManager,
  ) -> anyhow::Result<()> {
    // Handler for current session changes.
    manager.CurrentSessionChanged(&TypedEventHandler::new({
      let sender = self.event_sender.clone();
      move |manager: &Option<GsmtcManager>, _| {
        let session_id = manager
          .as_ref()
          .and_then(|manager| manager.GetCurrentSession().ok())
          .and_then(|session| session.SourceAppUserModelId().ok())
          .map(|id| id.to_string());

        sender
          .send(MediaSessionEvent::CurrentSessionChanged(session_id))
          .unwrap();

        Ok(())
      }
    }))?;

    // Handler for session list changes.
    manager.SessionsChanged(&TypedEventHandler::new({
      let sender = self.event_sender.clone();
      move |_, _| {
        sender.send(MediaSessionEvent::SessionListChanged).unwrap();
        Ok(())
      }
    }))?;

    Ok(())
  }

  /// Handles a media session event.
  fn handle_event(
    &mut self,
    event: MediaSessionEvent,
  ) -> anyhow::Result<()> {
    match event {
      MediaSessionEvent::CurrentSessionChanged(id) => {
        // TODO: Update `is_current_session` for all sessions.
        self.current_session_id = id;
      }
      MediaSessionEvent::SessionListChanged => {
        let manager = GsmtcManager::RequestAsync()?.get()?;
        self.update_all_sessions(&manager)?;
      }
      MediaSessionEvent::PlaybackInfoChanged(id) => {
        if let Some((session, _)) = self.sessions.get(&id) {
          self.update_session_playback(session, &id)?;
        }
      }
      MediaSessionEvent::MediaPropertiesChanged(id) => {
        if let Some((session, _)) = self.sessions.get(&id) {
          self.update_session_properties(session, &id)?;
        }
      }
      MediaSessionEvent::TimelinePropertiesChanged(id) => {
        if let Some((session, _)) = self.sessions.get(&id) {
          self.update_session_timeline(session, &id)?;
        }
      }
    }

    self.emit_output();

    Ok(())
  }

  /// Handles an incoming media provider function call.
  fn handle_function(
    &mut self,
    function: MediaFunction,
  ) -> anyhow::Result<ProviderFunctionResponse> {
    let session =
      self.session.as_ref().context("No active media session.")?;

    match function {
      MediaFunction::Play => {
        session.TryPlayAsync()?.get()?;
      }
      MediaFunction::Pause => {
        session.TryPauseAsync()?.get()?;
      }
      MediaFunction::TogglePlayPause => {
        session.TryTogglePlayPauseAsync()?.get()?;
      }
      MediaFunction::Next => {
        session.TrySkipNextAsync()?.get()?;
      }
      MediaFunction::Previous => {
        session.TrySkipPreviousAsync()?.get()?;
      }
    };

    Ok(ProviderFunctionResponse::Null)
  }

  /// Sets up a new media session when one becomes available.
  fn create_session(
    &mut self,
    manager: &GsmtcManager,
  ) -> anyhow::Result<()> {
    // Remove any existing session listeners.
    self.remove_session_listeners();

    // Get the updated session.
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
  ///
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
    self.common.emitter.emit_output(Ok(MediaOutput {
      current_session: None,
    }));
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
      self.emit_output(media_session);
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
      self.emit_output(media_session);
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
      self.emit_output(media_session);
    }
    Ok(())
  }

  /// Emits a `MediaSession` update through the provider's emitter.
  fn emit_output(&self, session: MediaSession) {
    self.common.emitter.emit_output(Ok(MediaOutput {
      current_session: Some(session),
    }));
  }

  /// Creates a complete MediaOutput struct from a Windows media session.
  fn to_media_output(
    session: &GsmtcSession,
  ) -> anyhow::Result<MediaOutput> {
    Ok(MediaOutput {
      current_session: Self::to_media_session_output(session)?,
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

  /// Updates media metadata properties in a `MediaSession`.
  fn update_media_properties(
    session: &mut MediaSession,
    properties: &GsmtcMediaProperties,
  ) -> anyhow::Result<()> {
    let artist = properties.Artist()?.to_string();
    let album_title = properties.AlbumTitle()?.to_string();
    let album_artist = properties.AlbumArtist()?.to_string();

    session.artist = (!artist.is_empty()).then_some(artist);
    session.album_title = (!album_title.is_empty()).then_some(album_title);
    session.album_artist =
      (!album_artist.is_empty()).then_some(album_artist);
    session.track_number = properties.TrackNumber()? as u32;

    Ok(())
  }

  /// Updates timeline properties (position/duration) in a `MediaSession`.
  fn update_timeline_properties(
    session: &mut MediaSession,
    properties: &GsmtcTimelineProperties,
  ) -> anyhow::Result<()> {
    session.start_time =
      properties.StartTime()?.Duration as u64 / 10_000_000;
    session.end_time = properties.EndTime()?.Duration as u64 / 10_000_000;
    session.position = properties.Position()?.Duration as u64 / 10_000_000;

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
