import { BarConfig } from './bar/bar-config.model';
import { GeneralConfig } from './general-config.model';

export interface UserConfig {
  general: GeneralConfig;
  [key: `bar/${string}`]: BarConfig;
}
