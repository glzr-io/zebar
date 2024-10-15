import type { Provider } from '../create-base-provider';

export interface CpuProviderConfig {
  type: 'cpu';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

export type CpuProvider = Provider<CpuProviderConfig, CpuOutput>;

export interface CpuOutput {
  frequency: number;
  usage: number;
  logicalCoreCount: number;
  physicalCoreCount: number;
  vendor: string;
}
