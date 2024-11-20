import type { DataSizeMeasure } from '~/utils';
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
  name: string | null;
  fileSystem: string;
  mountPoint: string;
  totalSpace: DataSizeMeasure;
  availableSpace: DataSizeMeasure;
  isRemovable: boolean;
  driveType: string;
}

export interface DiskOutput {
  disks: Disk[];
}
