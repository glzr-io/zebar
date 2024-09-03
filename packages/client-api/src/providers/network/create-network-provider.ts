import { z } from 'zod';

import {
  createBaseProvider,
  type Provider,
} from '../create-base-provider';
import { onProviderEmit } from '~/desktop';

export interface NetworkProviderConfig {
  type: 'network';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

const networkProviderConfigSchema = z.object({
  type: z.literal('network'),
  refreshInterval: z.coerce.number().default(5 * 1000),
});

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

export interface NetworkTraffic {
  received: number | null;
  transmitted: number | null;
}

export async function createNetworkProvider(
  config: NetworkProviderConfig,
): Promise<NetworkProvider> {
  const mergedConfig = networkProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<NetworkOutput>(mergedConfig, ({ variables }) => {
      if ('error' in variables) {
        queue.error(variables.error);
      } else {
        queue.value(variables.data);
      }
    });
  });
}
