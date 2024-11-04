import type { Provider } from '../create-base-provider';

export interface DiskProviderConfig {
  type: 'disk';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

export type DiskProvider = Provider<DiskProviderConfig, DiskOutput>;

export interface Disk {
  name: string;
  fileSystem: string;
  mountPoint: string;
  totalSpace: DiskSizeMeasure;
  availableSpace: DiskSizeMeasure;
  isRemovable: boolean;
  diskType: string;
}

export interface DiskOutput {
  disks: Disk[];
}

export interface DiskSizeMeasure {
  bytes: number;
  siValue: number;
  siUnit: string;
  iecValue: number;
  iecUnit: string;
}
