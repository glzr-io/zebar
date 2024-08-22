import type { Owner } from 'solid-js';

import { createProviderListener } from '../create-provider-listener';
import { ProviderType } from '../provider-type.model';

export interface CpuProviderConfig {
  type: ProviderType.CPU;

  refresh_interval: number;
}

export interface CpuVariables {
  frequency: number;
  usage: number;
  logicalCoreCount: number;
  physicalCoreCount: number;
  vendor: string;
}

export async function createCpuProvider(
  config: CpuProviderConfig,
  owner: Owner,
) {
  const cpuVariables = await createProviderListener<
    CpuProviderConfig,
    CpuVariables
  >(config, owner);

  return {
    get frequency() {
      return cpuVariables().frequency;
    },
    get usage() {
      return cpuVariables().usage;
    },
    get logicalCoreCount() {
      return cpuVariables().logicalCoreCount;
    },
    get physicalCoreCount() {
      return cpuVariables().physicalCoreCount;
    },
    get vendor() {
      return cpuVariables().vendor;
    },
  };
}
