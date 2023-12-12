import { MemoryProviderConfig } from '~/user-config';
import { memoize } from '~/utils';
import { createProviderListener } from '../create-provider-listener';

export interface MemoryVariables {
  isLoading: boolean;
  freeMemory: number;
  usedMemory: number;
  totalMemory: number;
  freeSwap: number;
  usedSwap: number;
  totalSwap: number;
}

export const createMemoryProvider = memoize((config: MemoryProviderConfig) => {
  const [memoryVariables] = createProviderListener<
    MemoryProviderConfig,
    MemoryVariables
  >(config);

  return {
    get isLoading() {
      return memoryVariables()?.isLoading ?? true;
    },
    get freeMemory() {
      return memoryVariables()?.freeMemory;
    },
    get usedMemory() {
      return memoryVariables()?.usedMemory;
    },
    get totalMemory() {
      return memoryVariables()?.totalMemory;
    },
    get freeSwap() {
      return memoryVariables()?.freeSwap;
    },
    get usedSwap() {
      return memoryVariables()?.usedSwap;
    },
    get totalSwap() {
      return memoryVariables()?.totalSwap;
    },
  };
});
