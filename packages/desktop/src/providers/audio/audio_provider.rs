use windows::{
  core::*,
  Win32::{
    Media::Audio::{
      eConsole, eRender, IMMDevice, IMMDeviceEnumerator,
      IMMNotificationClient, MMDeviceEnumerator,
    },
    System::Com::{
      CoCreateInstance, CoInitializeEx, CoUninitialize,
      StructuredStorage::PropVariantClear, CLSCTX_ALL,
      COINIT_MULTITHREADED,
    },
  },
};

fn main() -> Result<()> {
  // Initialize COM for the current thread
  unsafe { CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)? };

  // Create the device enumerator
  let device_enumerator: IMMDeviceEnumerator =
    unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)? };

  // Register the notification callback
  let callback = AudioDeviceNotificationCallback::new();
  let callback_interface: IMMNotificationClient = callback.into();

  unsafe {
    device_enumerator
      .RegisterEndpointNotificationCallback(&callback_interface)?
  };

  println!("Listening for audio device changes... Press Ctrl+C to exit.");

  // Keep the application running to listen for events
  loop {
    std::thread::sleep(std::time::Duration::from_secs(1));
  }

  // Normally, we would also unregister the callback and uninitialize COM
  // here, but this loop will run indefinitely, so those steps are
  // omitted.
}

struct AudioDeviceNotificationCallback;

impl AudioDeviceNotificationCallback {
  fn new() -> Self {
    Self {}
  }

  fn print_default_device_name(&self) -> Result<()> {
    // Create a new instance of IMMDeviceEnumerator to get the default
    // audio device
    let device_enumerator: IMMDeviceEnumerator =
      unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)? };
    let default_device = unsafe {
      device_enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?
    };

    // Retrieve the friendly name from the device's property store
    let property_store = unsafe {
      default_device
        .OpenPropertyStore(windows::Win32::System::Com::STGM_READ)?
    };
    let mut prop_value = PROPVARIANT::default();
    unsafe {
      property_store
        .GetValue(&PKEY_Device_FriendlyName, &mut prop_value)?
    };

    // Convert the PROPVARIANT to a Rust string and print it
    if let Some(friendly_name_ptr) =
      unsafe { prop_value.Anonymous.Anonymous.pwszVal.0.as_ref() }
    {
      let friendly_name =
        unsafe { widestring::U16CStr::from_ptr_str(friendly_name_ptr) }
          .to_string_lossy();
      println!(
        "Current default audio device changed to: {}",
        friendly_name
      );
    }

    // Clear the PROPVARIANT to prevent memory leaks
    unsafe { PropVariantClear(&mut prop_value)? };

    Ok(())
  }
}

impl IMMNotificationClient_Impl for AudioDeviceNotificationCallback {
  fn OnDefaultDeviceChanged(
    &self,
    _flow: windows::Win32::Media::Audio::EDataFlow,
    _role: windows::Win32::Media::Audio::ERole,
    _pwstr_device_id: &windows::core::PCWSTR,
  ) -> windows::Win32::Foundation::HRESULT {
    self
      .print_default_device_name()
      .unwrap_or_else(|e| eprintln!("Error: {:?}", e));
    windows::Win32::Foundation::S_OK
  }

  // Implement other methods to complete the interface, but leave them
  // empty if they’re not needed
  fn OnDeviceStateChanged(
    &self,
    _: &windows::core::PCWSTR,
    _: u32,
  ) -> windows::Win32::Foundation::HRESULT {
    windows::Win32::Foundation::S_OK
  }
  fn OnDeviceAdded(
    &self,
    _: &windows::core::PCWSTR,
  ) -> windows::Win32::Foundation::HRESULT {
    windows::Win32::Foundation::S_OK
  }
  fn OnDeviceRemoved(
    &self,
    _: &windows::core::PCWSTR,
  ) -> windows::Win32::Foundation::HRESULT {
    windows::Win32::Foundation::S_OK
  }
  fn OnPropertyValueChanged(
    &self,
    _: &windows::core::PCWSTR,
    _: &windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY,
  ) -> windows::Win32::Foundation::HRESULT {
    windows::Win32::Foundation::S_OK
  }
}
