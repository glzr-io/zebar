import { Transform, Type } from 'class-transformer';
import { ValidateNested } from 'class-validator';

import { BarConfig } from './bar/bar-config.model';
import { GeneralConfig } from './general-config.model';
import { toRecordType } from '~/shared/utils';

export class UserConfig {
  @Type(() => GeneralConfig)
  @ValidateNested()
  general: GeneralConfig;

  @Transform(toRecordType(BarConfig))
  @ValidateNested()
  bar: Record<string, BarConfig>;
}
