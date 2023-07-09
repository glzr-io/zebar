import { ScriptVariableConfig } from '../script-variable-config.model';

export interface ComponentConfigBase {
  id: string;
  class_name: string;
  type: string;
  style: string;
  template_variables: Record<string, string | ScriptVariableConfig>;
  template_commands: Record<string, string>;
  template: string;
  label: string;
}
