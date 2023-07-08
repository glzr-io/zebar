import { ScriptVariableConfig } from '../script-variable-config.model';
import { ComponentGroupConfig } from './component-group-config.model';

export interface BarConfig {
  style: string;
  template_variables: Record<string, string | ScriptVariableConfig>;
  template_commands: Record<string, string>;
  template: string;
  components_left: ComponentGroupConfig;
  components_center: ComponentGroupConfig;
  components_right: ComponentGroupConfig;
}
