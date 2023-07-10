import { Transform } from 'class-transformer';
import { ValidateNested } from 'class-validator';

import { ScriptVariableConfig } from '../script-variable-config.model';
import { ComponentGroupConfig } from './component-group-config.model';
import { toRecordType } from '~/shared/utils';

export class BarConfig {
  id: string;
  class_name: string;
  style: string;
  template_variables: Record<string, string | ScriptVariableConfig>;
  template_commands: Record<string, string>;
  template: string;

  @Transform(toRecordType(BarConfig))
  @ValidateNested()
  group: Record<string, ComponentGroupConfig>;
}
