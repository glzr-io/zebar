import { Type } from 'class-transformer';

import { ScriptVariableConfig } from '../script-variable-config.model';
import { ComponentGroupConfig } from './component-group-config.model';

export class BarConfig {
  id: string;
  class_name: string;
  style: string;
  template_variables: Record<string, string | ScriptVariableConfig>;
  template_commands: Record<string, string>;
  template: string;

  @Type(() => ComponentGroupConfig)
  group: Record<string, ComponentGroupConfig>;
}
