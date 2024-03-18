import type { Owner } from 'solid-js';

import type { NetworkProviderConfig } from '~/user-config';
import { createProviderListener } from '../create-provider-listener';

export interface NetworkVariables {
  defaultInterface: NetworkInterface | null;
  defaultGateway: NetworkGateway | null;
  interfaces: NetworkInterface[];
}

export interface NetworkInterface {
  name: string;
  friendlyName: string | null;
  description: string | null;
  type: InterfaceType;
  ipv4Addresses: string[];
  ipv6Addresses: string[];
  macAddress: string | null;
  transmitSeed: number | null;
  receiveSpeed: number | null;
  dnsServers: string[];
  isDefault: boolean;
}

export interface NetworkGateway {
  macAddress: string;
  ipv4Addresses: string[];
  ipv6Addresses: string[];
  ssid: string | null;
  signalStrength: number | null;
}

export enum InterfaceType {
  UNKNOWN = 'unknown',
  ETHERNET = 'ethernet',
  TOKEN_RING = 'token_ring',
  FDDI = 'fddi',
  PPP = 'ppp',
  LOOPBACK = 'loopback',
  SLIP = 'slip',
  ATM = 'atm',
  GENERIC_MODEM = 'generic_modem',
  ISDN = 'isdn',
  WIFI = 'wifi',
  DSL = 'dsl',
  TUNNEL = 'tunnel',
  HIGH_PERFORMANCE_SERIAL_BUS = 'high_performance_serial_bus',
  MOBILE_BROADBAND = 'mobile_broadband',
  BRIDGE = 'bridge',
}

export async function createNetworkProvider(
  config: NetworkProviderConfig,
  owner: Owner,
) {
  const networkVariables = await createProviderListener<
    NetworkProviderConfig,
    NetworkVariables
  >(config, owner);

  return {
    get defaultInterface() {
      return networkVariables().defaultInterface;
    },
    get defaultGateway() {
      return networkVariables().defaultGateway;
    },
    get interfaces() {
      return networkVariables().interfaces;
    },
  };
}
