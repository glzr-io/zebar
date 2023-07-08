import { ScriptVariableConfig } from '../script-variable-config.model';

export interface ComponentConfigBase {
  type: string;
  style: string;
  template_variables: Record<string, string | ScriptVariableConfig>;
  template_commands: Record<string, string>;
  template: string;
  label: string;
}
