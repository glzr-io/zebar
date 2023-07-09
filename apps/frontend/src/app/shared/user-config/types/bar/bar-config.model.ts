import { ScriptVariableConfig } from '../script-variable-config.model';
import { ComponentGroupConfig } from './component-group-config.model';

export interface BarConfig {
  id: string;
  class_name: string;
  style: string;
  template_variables: Record<string, string | ScriptVariableConfig>;
  template_commands: Record<string, string>;
  template: string;
  [key: `group/${string}`]: ComponentGroupConfig;
}
