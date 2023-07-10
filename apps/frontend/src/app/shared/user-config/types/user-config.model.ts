import { Transform, Type } from 'class-transformer';
import { ValidateNested } from 'class-validator';

import { BarConfig } from './bar/bar-config.model';
import { GeneralConfig } from './general-config.model';
import { toRecordType, ValidateRecord } from '~/shared/utils';

export class UserConfig {
  @Type(() => GeneralConfig)
  @ValidateNested()
  general: GeneralConfig;

  @Transform(toRecordType(BarConfig))
  @ValidateRecord()
  bar: Record<string, BarConfig>;
}
