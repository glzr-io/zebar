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
    GlobalSystemMediaTransportControlsSessionPlaybackInfo as GsmtcPlaybackInfo,
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

impl Default for MediaSession {
  fn default() -> Self {
    Self {
      session_id: "".to_string(),
      title: "".to_string(),
      artist: None,
      album_title: None,
      album_artist: None,
      track_number: 0,
      start_time: 0,
      end_time: 0,
      position: 0,
      is_playing: false,
      is_current_session: false,
    }
  }
}

/// Events that can be emitted from media session state changes.
#[derive(Debug)]
enum MediaSessionEvent {
  SessionAddOrRemove,
  CurrentSessionChanged(Option<String>),
  PlaybackInfoChanged(String),
  MediaPropertiesChanged(String),
  TimelinePropertiesChanged(String),
}

/// Holds event registration tokens for media session callbacks.
///
/// These need to be cleaned up when the session changes.
#[derive(Debug)]
struct EventTokens {
  playback: EventRegistrationToken,
  properties: EventRegistrationToken,
  timeline: EventRegistrationToken,
}

/// Holds the state of a media session.
#[derive(Debug)]
struct SessionState {
  session: GsmtcSession,
  tokens: EventTokens,
  output: MediaSession,
}

pub struct MediaProvider {
  common: CommonProviderState,
  current_session_id: Option<String>,
  session_states: HashMap<String, SessionState>,
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
      session_states: HashMap::new(),
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
  /// - `SessionAddOrRemove`: for when the list of available media sessions
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

    // Handler for a session is added or removed.
    manager.SessionsChanged(&TypedEventHandler::new({
      let sender = self.event_sender.clone();
      move |_, _| {
        sender.send(MediaSessionEvent::SessionAddOrRemove).unwrap();
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
      MediaSessionEvent::SessionAddOrRemove => {
        let manager = GsmtcManager::RequestAsync()?.get()?;
        self.update_all_sessions(&manager)?;
      }
      MediaSessionEvent::PlaybackInfoChanged(id) => {
        if let Some((session, _)) = self.session_states.get(&id) {
          self.update_session_playback(session, &id)?;
        }
      }
      MediaSessionEvent::MediaPropertiesChanged(id) => {
        if let Some((session, _)) = self.session_states.get(&id) {
          self.update_session_properties(session, &id)?;
        }
      }
      MediaSessionEvent::TimelinePropertiesChanged(id) => {
        if let Some((session, _)) = self.session_states.get(&id) {
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
    let session_state = self
      .current_session_id
      .as_ref()
      .and_then(|id| self.session_states.get(id))
      .context("No active media session.")?;

    match function {
      MediaFunction::Play(args) => {
        session_state.session.TryPlayAsync()?.get()?;
      }
      MediaFunction::Pause(args) => {
        session_state.session.TryPauseAsync()?.get()?;
      }
      MediaFunction::TogglePlayPause(args) => {
        session_state.session.TryTogglePlayPauseAsync()?.get()?;
      }
      MediaFunction::Next(args) => {
        session_state.session.TrySkipNextAsync()?.get()?;
      }
      MediaFunction::Previous(args) => {
        session_state.session.TrySkipPreviousAsync()?.get()?;
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
    session_id: &str,
  ) -> anyhow::Result<EventTokens> {
    let session_id = session_id.to_string();

    Ok(EventTokens {
      playback: session.PlaybackInfoChanged(&TypedEventHandler::new({
        let sender = self.event_sender.clone();
        let session_id = session_id.clone();
        move |_, _| {
          sender
            .send(MediaSessionEvent::PlaybackInfoChanged(
              session_id.clone(),
            ))
            .unwrap();
          Ok(())
        }
      }))?,
      properties: session.MediaPropertiesChanged(
        &TypedEventHandler::new({
          let sender = self.event_sender.clone();
          let session_id = session_id.clone();
          move |_, _| {
            sender
              .send(MediaSessionEvent::MediaPropertiesChanged(
                session_id.clone(),
              ))
              .unwrap();
            Ok(())
          }
        }),
      )?,
      timeline: session.TimelinePropertiesChanged(
        &TypedEventHandler::new({
          let sender = self.event_sender.clone();
          let session_id = session_id.clone();
          move |_, _| {
            sender
              .send(MediaSessionEvent::TimelinePropertiesChanged(
                session_id.clone(),
              ))
              .unwrap();
            Ok(())
          }
        }),
      )?,
    })
  }

  /// Cleans up event listeners from the given session.
  fn remove_session_listeners(
    session: &GsmtcSession,
    tokens: &EventTokens,
  ) {
    let _ = session.RemovePlaybackInfoChanged(tokens.playback);
    let _ = session.RemoveMediaPropertiesChanged(tokens.properties);
    let _ = session.RemoveTimelinePropertiesChanged(tokens.timeline);
  }

  /// Emits a `MediaOutput` update through the provider's emitter.
  fn emit_output(&self) {
    // At times, GSMTC can have a valid session, but return empty string
    // for all media properties. Check that we at least have a valid
    // title, otherwise, return `None`.
    let current_session = self
      .current_session_id
      .as_ref()
      .and_then(|id| {
        self
          .session_states
          .get(id)
          .filter(|state| !state.output.title.is_empty())
      })
      .map(|state| state.output.clone());

    let all_sessions = self
      .session_states
      .values()
      .filter(|state| !state.output.title.is_empty())
      .map(|state| state.output.clone())
      .collect();

    self.common.emitter.emit_output(Ok(MediaOutput {
      current_session,
      all_sessions,
    }));
  }

  /// Creates a `MediaSession` from a Windows media session.
  fn to_media_session_output(
    session: &GsmtcSession,
  ) -> anyhow::Result<MediaSession> {
    let mut session_output = MediaSession::default();

    Self::update_media_properties(
      &mut session_output,
      &session.TryGetMediaPropertiesAsync()?.get()?,
    )?;
    Self::update_timeline_properties(
      &mut session_output,
      &session.GetTimelineProperties()?,
    )?;
    Self::update_playback_info(
      &mut session_output,
      &session.GetPlaybackInfo()?,
    )?;

    Ok(session_output)
  }

  /// Updates media metadata properties in a `MediaSession`.
  fn update_media_properties(
    session_output: &mut MediaSession,
    properties: &GsmtcMediaProperties,
  ) -> anyhow::Result<()> {
    let artist = properties.Artist()?.to_string();
    let album_title = properties.AlbumTitle()?.to_string();
    let album_artist = properties.AlbumArtist()?.to_string();

    session_output.artist = (!artist.is_empty()).then_some(artist);
    session_output.album_title =
      (!album_title.is_empty()).then_some(album_title);
    session_output.album_artist =
      (!album_artist.is_empty()).then_some(album_artist);
    session_output.track_number = properties.TrackNumber()? as u32;

    Ok(())
  }

  /// Updates timeline properties (position/duration) in a `MediaSession`.
  fn update_timeline_properties(
    session_output: &mut MediaSession,
    properties: &GsmtcTimelineProperties,
  ) -> anyhow::Result<()> {
    session_output.start_time =
      properties.StartTime()?.Duration as u64 / 10_000_000;
    session_output.end_time =
      properties.EndTime()?.Duration as u64 / 10_000_000;
    session_output.position =
      properties.Position()?.Duration as u64 / 10_000_000;

    Ok(())
  }

  /// Updates playback info in a `MediaSession`.
  fn update_playback_info(
    session_output: &mut MediaSession,
    playback_info: &GsmtcPlaybackInfo,
  ) -> anyhow::Result<()> {
    session_output.is_playing =
      playback_info.PlaybackStatus()? == GsmtcPlaybackStatus::Playing;

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
