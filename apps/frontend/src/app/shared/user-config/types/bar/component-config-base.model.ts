import { IsIn } from 'class-validator';

import { ScriptVariableConfig } from '../script-variable-config.model';

export abstract class ComponentConfigBase {
  id: string;
  class_name: string;
  @IsIn(['cpu', 'glazewm', 'clock'])
  type: string;
  style: string;
  template_variables: Record<string, string | ScriptVariableConfig>;
  template_commands: Record<string, string>;
  template: string;
  label: string;
}
