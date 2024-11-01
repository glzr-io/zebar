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
  totalSpace: number;
  availableSpace: number;
  isRemovable: boolean;
  diskType: string;
}

export interface DiskOutput {
  disks: Disk[];
}
