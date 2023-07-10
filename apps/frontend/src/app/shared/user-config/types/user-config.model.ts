import { Transform, Type } from 'class-transformer';

import { BarConfig } from './bar/bar-config.model';
import { GeneralConfig } from './general-config.model';
import { toRecordType } from '~/shared/utils';

export class UserConfig {
  @Type(() => GeneralConfig)
  general: GeneralConfig;

  @Transform(toRecordType(BarConfig))
  bar: Record<string, BarConfig>;
}
