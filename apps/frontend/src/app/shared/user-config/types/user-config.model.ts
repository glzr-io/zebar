import { Type } from 'class-transformer';

import { BarConfig } from './bar/bar-config.model';
import { GeneralConfig } from './general-config.model';

export class UserConfig {
  @Type(() => GeneralConfig)
  general: GeneralConfig;

  @Type(() => BarConfig)
  bar: Record<string, BarConfig>;
}
