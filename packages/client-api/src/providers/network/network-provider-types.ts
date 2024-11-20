import type { DataSizeMeasure } from '~/utils';
import type { Provider } from '../create-base-provider';

export interface NetworkProviderConfig {
  type: 'network';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

export type NetworkProvider = Provider<
  NetworkProviderConfig,
  NetworkOutput
>;

export interface NetworkOutput {
  defaultInterface: NetworkInterface | null;
  defaultGateway: NetworkGateway | null;
  interfaces: NetworkInterface[];
  traffic: NetworkTraffic | null;
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

export type InterfaceType =
  | 'unknown'
  | 'ethernet'
  | 'token_ring'
  | 'fddi'
  | 'ppp'
  | 'loopback'
  | 'slip'
  | 'atm'
  | 'generic_modem'
  | 'isdn'
  | 'wifi'
  | 'dsl'
  | 'tunnel'
  | 'high_performance_serial_bus'
  | 'mobile_broadband'
  | 'bridge';

export interface NetworkTraffic {
  received: DataSizeMeasure;
  totalReceived: DataSizeMeasure;
  transmitted: DataSizeMeasure;
  totalTransmitted: DataSizeMeasure;
}
