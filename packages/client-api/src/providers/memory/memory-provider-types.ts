import type { Provider } from '../create-base-provider';

export interface MemoryProviderConfig {
  type: 'memory';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

export type MemoryProvider = Provider<MemoryProviderConfig, MemoryOutput>;

export interface MemoryOutput {
  usage: number;
  freeMemory: number;
  usedMemory: number;
  totalMemory: number;
  freeSwap: number;
  usedSwap: number;
  totalSwap: number;
}
