use netdev::interface::InterfaceType as NdInterfaceType;
use netdev::ip::{Ipv4Net as NdIpv4Net, Ipv6Net as NdIpv6Net};
use netdev::mac::MacAddr as NdMacAddr;
use serde::{Serialize, Serializer};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetworkVariables {
  pub default_interface: NetworkInterface,
  pub default_gateway: Gateway,
  pub interfaces: Vec<NetworkInterface>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInterface {
  pub name: String,
  pub friendly_name: Option<String>,
  pub description: Option<String>,
  #[serde(serialize_with = "interfacetype_ser")]
  #[serde(rename = "type")]
  pub interface_type: NdInterfaceType,
  #[serde(serialize_with = "ipv4_ser")]
  pub ipv4: Vec<NdIpv4Net>,
  #[serde(serialize_with = "ipv6_ser")]
  pub ipv6: Vec<NdIpv6Net>,
  #[serde(serialize_with = "macaddr_ser")]
  pub mac_address: NdMacAddr,
  pub transmit_speed: Option<u64>,
  pub receive_speed: Option<u64>,
  pub dns_servers: Vec<IpAddr>,
  pub is_default: bool,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Gateway {
  #[serde(serialize_with = "macaddr_ser")]
  pub mac_address: NdMacAddr,
  pub ipv4_addresses: Vec<Ipv4Addr>,
  pub ipv6_addresses: Vec<Ipv6Addr>,
  pub ssid: String,
  pub signal_strength_percent: u32,
  pub is_connected: bool,
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

fn itype_to_local(itype: &NdInterfaceType) -> InterfaceType {
  match itype {
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

fn interfacetype_ser<S: Serializer>(
  itype: &NdInterfaceType,
  serializer: S,
) -> Result<S::Ok, S::Error> {
  let local_itype = itype_to_local(itype);
  local_itype.serialize(serializer)
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ipv4Net {
  pub address: Ipv4Addr,
  pub prefix_length: u8,
  pub netmask: Ipv4Addr,
}

fn ipv4net_to_local(ipv4net: &Vec<NdIpv4Net>) -> Vec<Ipv4Net> {
  let mut result = Vec::new();
  for net in ipv4net {
    match net {
      NdIpv4Net {
        addr,
        prefix_len,
        netmask,
      } => {
        result.push(Ipv4Net {
          address: *addr,
          prefix_length: *prefix_len,
          netmask: *netmask,
        });
      }
    }
  }
  result
}

fn ipv4_ser<S: Serializer>(
  ipv4net: &Vec<NdIpv4Net>,
  serializer: S,
) -> Result<S::Ok, S::Error> {
  let local_ipv4net = ipv4net_to_local(ipv4net);
  local_ipv4net.serialize(serializer)
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Ipv6Net {
  pub address: Ipv6Addr,
  pub prefix_length: u8,
  pub netmask: Ipv6Addr,
}

fn ipv6net_to_local(ipv6net: &Vec<NdIpv6Net>) -> Vec<Ipv6Net> {
  let mut result = Vec::new();
  for net in ipv6net {
    match net {
      NdIpv6Net {
        addr,
        prefix_len,
        netmask,
      } => {
        result.push(Ipv6Net {
          address: *addr,
          prefix_length: *prefix_len,
          netmask: *netmask,
        });
      }
    }
  }
  result
}

fn ipv6_ser<S: Serializer>(
  ipv6net: &Vec<NdIpv6Net>,
  serializer: S,
) -> Result<S::Ok, S::Error> {
  let local_ipv6net = ipv6net_to_local(ipv6net);
  local_ipv6net.serialize(serializer)
}

#[derive(Serialize, Debug, Clone)]
pub struct MacAddr(pub u8, pub u8, pub u8, pub u8, pub u8, pub u8);

fn macaddr_to_local(macaddr: &NdMacAddr) -> Option<MacAddr> {
  match macaddr {
    NdMacAddr { .. } => {
      let octets = macaddr.octets();
      Some(MacAddr(
        octets[0], octets[1], octets[2], octets[3], octets[4], octets[5],
      ))
    }
  }
}

fn macaddr_ser<S: Serializer>(
  macaddr: &NdMacAddr,
  serializer: S,
) -> Result<S::Ok, S::Error> {
  let local_macaddr = macaddr_to_local(macaddr);
  local_macaddr.serialize(serializer)
}
