import type { Provider } from '../create-base-provider';

export interface IpProviderConfig {
  type: 'ip';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

export type IpProvider = Provider<IpProviderConfig, IpOutput>;

export interface IpOutput {
  address: string;
  approxCity: string;
  approxCountry: string;
  approxLatitude: number;
  approxLongitude: number;
}
