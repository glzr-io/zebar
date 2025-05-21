use netdev::interface::InterfaceType as NdInterfaceType;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkTraffic {
  pub received: NetworkTrafficMeasure,
  pub total_received: NetworkTrafficMeasure,
  pub transmitted: NetworkTrafficMeasure,
  pub total_transmitted: NetworkTrafficMeasure,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkTrafficMeasure {
  pub bytes: u64,
  pub si_value: f64,
  pub si_unit: String,
  pub iec_value: f64,
  pub iec_unit: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInterface {
  pub name: String,
  pub friendly_name: Option<String>,
  pub description: Option<String>,
  #[serde(rename = "type")]
  pub interface_type: InterfaceType,
  pub ipv4_addresses: Vec<String>,
  pub ipv6_addresses: Vec<String>,
  pub mac_address: Option<String>,
  pub transmit_speed: Option<u64>,
  pub receive_speed: Option<u64>,
  pub dns_servers: Vec<String>,
  pub is_default: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkGateway {
  pub mac_address: String,
  pub ipv4_addresses: Vec<String>,
  pub ipv6_addresses: Vec<String>,
  pub ssid: Option<String>,
  pub signal_strength: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InterfaceType {
  Unknown,
  Ethernet,
  TokenRing,
  Fddi,
  Ppp,
  Loopback,
  Slip,
  Atm,
  GenericModem,
  ProprietaryVirtual,
  Isdn,
  Wifi,
  Dsl,
  Tunnel,
  HighPerformanceSerialBus,
  MobileBroadband,
  Bridge,
  Can,
}

impl From<NdInterfaceType> for InterfaceType {
  fn from(layout: NdInterfaceType) -> Self {
    match layout {
      NdInterfaceType::Unknown => InterfaceType::Unknown,
      NdInterfaceType::Ethernet
      | NdInterfaceType::Ethernet3Megabit
      | NdInterfaceType::FastEthernetFx
      | NdInterfaceType::FastEthernetT
      | NdInterfaceType::GigabitEthernet => InterfaceType::Ethernet,
      NdInterfaceType::TokenRing => InterfaceType::TokenRing,
      NdInterfaceType::Fddi => InterfaceType::Fddi,
      NdInterfaceType::Ppp => InterfaceType::Ppp,
      NdInterfaceType::Loopback => InterfaceType::Loopback,
      NdInterfaceType::Slip => InterfaceType::Slip,
      NdInterfaceType::Atm | NdInterfaceType::IPOverAtm => {
        InterfaceType::Atm
      }
      NdInterfaceType::GenericModem => InterfaceType::GenericModem,
      NdInterfaceType::ProprietaryVirtual => {
        InterfaceType::ProprietaryVirtual
      }
      NdInterfaceType::Isdn
      | NdInterfaceType::BasicIsdn
      | NdInterfaceType::PrimaryIsdn => InterfaceType::Isdn,
      NdInterfaceType::Wireless80211 => InterfaceType::Wifi,
      NdInterfaceType::AsymmetricDsl
      | NdInterfaceType::RateAdaptDsl
      | NdInterfaceType::SymmetricDsl
      | NdInterfaceType::VeryHighSpeedDsl
      | NdInterfaceType::MultiRateSymmetricDsl => InterfaceType::Dsl,
      NdInterfaceType::Tunnel => InterfaceType::Tunnel,
      NdInterfaceType::HighPerformanceSerialBus => {
        InterfaceType::HighPerformanceSerialBus
      }
      NdInterfaceType::Wman
      | NdInterfaceType::Wwanpp
      | NdInterfaceType::Wwanpp2 => InterfaceType::MobileBroadband,
      NdInterfaceType::Bridge => InterfaceType::Bridge,
      NdInterfaceType::Can => InterfaceType::Can,
      NdInterfaceType::UnknownWithValue(_u32) => InterfaceType::Unknown,
    }
  }
}
