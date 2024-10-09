use std::ffi::c_void;

use anyhow::Context;
#[cfg(target_os = "windows")]
use windows::Win32::{
  Foundation::{HANDLE, INVALID_HANDLE_VALUE, WIN32_ERROR},
  NetworkManagement::WiFi::{
    wlan_intf_opcode_current_connection, WlanCloseHandle,
    WlanEnumInterfaces, WlanFreeMemory, WlanOpenHandle,
    WlanQueryInterface, WLAN_CONNECTION_ATTRIBUTES,
  },
};

#[derive(Debug)]
pub struct WifiHotstop {
  pub ssid: Option<String>,
  pub signal_strength: Option<u32>,
}

#[derive(Debug)]
struct WlanHandle(HANDLE);

impl Drop for WlanHandle {
  fn drop(&mut self) {
    unsafe {
      WlanCloseHandle(self.0, None);
    }
  }
}

/// Gets wifi ssid and signal stregth using winapi
pub fn default_gateway_wifi() -> anyhow::Result<WifiHotstop> {
  #[cfg(not(target_os = "windows"))]
  {
    Ok(WifiHotstop {
      ssid: None,
      signal_strength: None,
    })
  }
  #[cfg(target_os = "windows")]
  {
    let mut pdw_negotiated_version = 0;
    let mut wlan_handle = WlanHandle(INVALID_HANDLE_VALUE);

    WIN32_ERROR(unsafe {
      WlanOpenHandle(
        2,
        None,
        &mut pdw_negotiated_version,
        &mut wlan_handle.0,
      )
    })
    .ok()
    .context("Failed to open Wlan handle")?;

    let mut wlan_interface_info_list = std::ptr::null_mut();
    WIN32_ERROR(unsafe {
      WlanEnumInterfaces(
        wlan_handle.0,
        None,
        &mut wlan_interface_info_list,
      )
    })
    .ok()
    .context("Failed to get Wlan interfaces")?;

    let guid = (unsafe { *wlan_interface_info_list }).InterfaceInfo[0]
      .InterfaceGuid;
    unsafe { WlanFreeMemory(wlan_interface_info_list as *mut c_void) };

    let mut data_size = 0;
    let mut pdata = std::ptr::null_mut();

    WIN32_ERROR(unsafe {
      WlanQueryInterface(
        wlan_handle.0,
        &guid,
        wlan_intf_opcode_current_connection,
        None,
        &mut data_size,
        &mut pdata,
        None,
      )
    })
    .ok()
    .context("Failed to get connected Wlan interface")?;

    let wlan_connection_atributes =
      pdata as *mut WLAN_CONNECTION_ATTRIBUTES;
    let atributes =
      unsafe { *wlan_connection_atributes }.wlanAssociationAttributes;

    unsafe { WlanFreeMemory(pdata) };

    // needed to remove leading zeros in array
    let ssid_arr = atributes.dot11Ssid.ucSSID;
    let mut ssid_vec = ssid_arr
      .into_iter()
      .rev()
      .skip_while(|&byte| byte == 0)
      .collect::<Vec<_>>();
    ssid_vec.reverse();
    let ssid = String::from_utf8(ssid_vec).unwrap();

    Ok(WifiHotstop {
      ssid: Some(ssid),
      signal_strength: Some(atributes.wlanSignalQuality),
    })
  }
}
