import { CpuProviderConfig } from '~/user-config';
import { memoize } from '~/utils';
import { createProviderListener } from '../create-provider-listener';

export interface CpuVariables {
  isLoading: boolean;
  frequency: number;
  usage: number;
  logicalCoreCount: number;
  physicalCoreCount: number;
  vendor: string;
}

export const createCpuProvider = memoize((config: CpuProviderConfig) => {
  const [cpuVariables] = createProviderListener<
    CpuProviderConfig,
    CpuVariables
  >(config);

  return {
    get isLoading() {
      return cpuVariables()?.isLoading ?? true;
    },
    get frequency() {
      return cpuVariables()?.frequency;
    },
    get usage() {
      return cpuVariables()?.usage;
    },
    get logicalCoreCount() {
      return cpuVariables()?.logicalCoreCount;
    },
    get physicalCoreCount() {
      return cpuVariables()?.physicalCoreCount;
    },
    get vendor() {
      return cpuVariables()?.vendor;
    },
  };
});
