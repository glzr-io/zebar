import { Transform } from 'class-transformer';

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
  group: Record<string, ComponentGroupConfig>;
}
