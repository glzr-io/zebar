import type { Provider } from '../create-base-provider';

export interface GpuProviderConfig {
  type: 'gpu';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

export type GpuProvider = Provider<GpuProviderConfig, GpuOutput>;

export interface GpuOutput {
  name: string;
  temperature: number | null;
  usage: number | null;
  memoryUsed: number | null;
  memoryTotal: number | null;
}
