import { ComponentConfigBase } from '../component-config-base.model';

export interface CpuComponentConfig extends ComponentConfigBase {
  type: 'cpu';
  refresh_interval_ms: number;
}
