import type { Owner } from 'solid-js';

import type { NewNetworkProviderConfig } from '~/user-config';
import { createProviderListener } from '../create-provider-listener';

export interface NewNetworkVariables {
  defaultInterface: NetworkInterface;
  defaultGateway: Gateway;
  interfaces: NetworkInterface[];
}

export async function createNewNetworkProvider(
  config: NewNetworkProviderConfig,
  owner: Owner,
) {
  const newNetworkVariables = await createProviderListener<
    NewNetworkProviderConfig,
    NewNetworkVariables
  >(config, owner);

  return {
    get defaultInterface() {
      return newNetworkVariables().defaultInterface;
    },
    get defaultGateway() {
      return newNetworkVariables().defaultGateway;
    },
    get interfaces() {
      return newNetworkVariables().interfaces;
    },
  };
}

export interface NetworkInterface {
  name: string;
  friendlName: string;
  description: string;
  interfaceType: InterfaceType;
  ipv4: Ipv4Net;
  ipv6: Ipv6Net;
  macAddress: MacAddress;
  transmitSeed: number;
  receiveSpeed: number;
  dnsServers: (Ipv4Addr | Ipv6Addr)[];
  default: boolean;
}

export interface Gateway {
  macAddress: MacAddress;
  ipv4: Ipv4Addr[];
  ipv6: Ipv6Addr[];
  ssid: string;
  signal_strength: number;
  connected: boolean;
}

enum InterfaceType {
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

interface Ipv4Net {
  addr: Ipv4Addr;
  netmask: Ipv4Addr;
  prefixLength: number;
}

interface Ipv6Net {
  addr: Ipv6Addr;
  netmask: Ipv6Addr;
  prefixLength: number;
}

interface Ipv6Addr {
  octects: number[];
}

interface Ipv4Addr {
  octects: number[];
}

interface MacAddress {
  octects: number[];
}
