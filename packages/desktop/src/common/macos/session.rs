use std::ptr::NonNull;

use block2::RcBlock;
use objc2::{
  class, msg_send,
  rc::Retained,
  runtime::{AnyObject, ProtocolObject},
};
use objc2_foundation::{
  NSNotification, NSNotificationCenter, NSObjectProtocol, NSString,
};
use tokio::sync::mpsc;

/// A notification center paired with the token of one registration.
type Registration = (
  Retained<NSNotificationCenter>,
  Retained<ProtocolObject<dyn NSObjectProtocol>>,
);

/// Observes macOS wake and unlock notifications for the lifetime of the
/// application.
pub struct SessionObserver {
  registrations: Vec<Registration>,
}

// SAFETY: Centers support registration removal on any thread. Tokens are
// retained opaque registration handles. Callbacks only use a thread-safe
// channel sender, and application state exposes no access to these tokens.
unsafe impl Send for SessionObserver {}
unsafe impl Sync for SessionObserver {}

impl SessionObserver {
  /// Registers on the main thread. Callbacks only enqueue recovery work.
  pub fn new() -> (Self, mpsc::UnboundedReceiver<()>) {
    let (tx, rx) = mpsc::unbounded_channel();
    // SAFETY: Setup runs on the AppKit main thread. Both notification
    // centers copy their blocks and retain the registration handles.
    unsafe {
      let workspace: Retained<AnyObject> =
        msg_send![class!(NSWorkspace), sharedWorkspace];
      let center: Retained<NSNotificationCenter> =
        msg_send![&*workspace, notificationCenter];
      // NSDistributedNotificationCenter inherits NSNotificationCenter.
      let distributed: Retained<NSNotificationCenter> =
        msg_send![class!(NSDistributedNotificationCenter), defaultCenter];
      let mut registrations = Vec::new();
      for (center, names) in [
        (
          center,
          &[
            "NSWorkspaceDidWakeNotification",
            "NSWorkspaceScreensDidWakeNotification",
            "NSWorkspaceSessionDidBecomeActiveNotification",
          ][..],
        ),
        // macOS has no public notification specifically for unlocking.
        (distributed, &["com.apple.screenIsUnlocked"][..]),
      ] {
        for name in names {
          let tx = tx.clone();
          let callback =
            RcBlock::new(move |_: NonNull<NSNotification>| {
              let _ = tx.send(());
            });
          let token = center.addObserverForName_object_queue_usingBlock(
            Some(&NSString::from_str(name)),
            None,
            None,
            &callback,
          );
          registrations.push((center.clone(), token));
        }
      }
      (Self { registrations }, rx)
    }
  }
}

impl Drop for SessionObserver {
  fn drop(&mut self) {
    for (center, token) in &self.registrations {
      // SAFETY: The retained token remains valid until after removal.
      unsafe {
        center.removeObserver(token.as_ref());
      }
    }
  }
}
