use windows::Win32::System::Com::{
  CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED,
  COINIT_MULTITHREADED,
};

thread_local! {
  /// Manages per-thread COM initialization. COM must be initialized on each
  /// thread that uses it, so we store this in thread-local storage to handle
  /// the setup and cleanup automatically.
  pub static COM_INIT: ComInit = ComInit::new();
}

pub struct ComInit();

impl ComInit {
  /// Initializes COM on the current thread with apartment threading model.
  /// `COINIT_APARTMENTTHREADED` is required for shell COM objects.
  ///
  /// # Panics
  ///
  /// Panics if COM initialization fails. This is typically only possible
  /// if COM is already initialized with an incompatible threading model.
  pub fn new() -> Self {
    unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }
      .ok()
      .expect("Unable to initialize COM.");

    Self()
  }
}

impl Drop for ComInit {
  fn drop(&mut self) {
    unsafe { CoUninitialize() };
  }
}
