use netdev::interface::InterfaceType as NdInterfaceType;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetworkVariables {
  pub default_interface: NetworkInterface,
  pub default_gateway: Option<NetworkGateway>,
  pub interfaces: Vec<NetworkInterface>,
}

#[derive(Serialize, Debug, Clone)]
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

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetworkGateway {
  pub mac_address: String,
  pub ipv4_addresses: Vec<String>,
  pub ipv6_addresses: Vec<String>,
  pub ssid: Option<String>,
  pub signal_strength: Option<u32>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum InterfaceType {
  Unknown,
  Ethernet,
  TokenRing,
  Fddi,
  BasicIsdn,
  PrimaryIsdn,
  Ppp,
  Loopback,
  Ethernet3Megabit,
  Slip,
  Atm,
  GenericModem,
  FastEthernetT,
  Isdn,
  FastEthernetFx,
  Wireless80211,
  AsymmetricDsl,
  RateAdaptDsl,
  SymmetricDsl,
  VeryHighSpeedDsl,
  IPOverAtm,
  GigabitEthernet,
  Tunnel,
  MultiRateSymmetricDsl,
  HighPerformanceSerialBus,
  Wman,
  Wwanpp,
  Wwanpp2,
  Bridge,
}

impl From<NdInterfaceType> for InterfaceType {
  fn from(layout: NdInterfaceType) -> Self {
    match layout {
      NdInterfaceType::Unknown => InterfaceType::Unknown,
      NdInterfaceType::Ethernet => InterfaceType::Ethernet,
      NdInterfaceType::TokenRing => InterfaceType::TokenRing,
      NdInterfaceType::Fddi => InterfaceType::Fddi,
      NdInterfaceType::BasicIsdn => InterfaceType::BasicIsdn,
      NdInterfaceType::PrimaryIsdn => InterfaceType::PrimaryIsdn,
      NdInterfaceType::Ppp => InterfaceType::Ppp,
      NdInterfaceType::Loopback => InterfaceType::Loopback,
      NdInterfaceType::Ethernet3Megabit => InterfaceType::Ethernet3Megabit,
      NdInterfaceType::Slip => InterfaceType::Slip,
      NdInterfaceType::Atm => InterfaceType::Atm,
      NdInterfaceType::GenericModem => InterfaceType::GenericModem,
      NdInterfaceType::FastEthernetT => InterfaceType::FastEthernetT,
      NdInterfaceType::Isdn => InterfaceType::Isdn,
      NdInterfaceType::FastEthernetFx => InterfaceType::FastEthernetFx,
      NdInterfaceType::Wireless80211 => InterfaceType::Wireless80211,
      NdInterfaceType::AsymmetricDsl => InterfaceType::AsymmetricDsl,
      NdInterfaceType::RateAdaptDsl => InterfaceType::RateAdaptDsl,
      NdInterfaceType::SymmetricDsl => InterfaceType::SymmetricDsl,
      NdInterfaceType::VeryHighSpeedDsl => InterfaceType::VeryHighSpeedDsl,
      NdInterfaceType::IPOverAtm => InterfaceType::IPOverAtm,
      NdInterfaceType::GigabitEthernet => InterfaceType::GigabitEthernet,
      NdInterfaceType::Tunnel => InterfaceType::Tunnel,
      NdInterfaceType::MultiRateSymmetricDsl => {
        InterfaceType::MultiRateSymmetricDsl
      }
      NdInterfaceType::HighPerformanceSerialBus => {
        InterfaceType::HighPerformanceSerialBus
      }
      NdInterfaceType::Wman => InterfaceType::Wman,
      NdInterfaceType::Wwanpp => InterfaceType::Wwanpp,
      NdInterfaceType::Wwanpp2 => InterfaceType::Wwanpp2,
      NdInterfaceType::Bridge => InterfaceType::Bridge,
    }
  }
}
