import { ComponentConfig } from './component-config.model';
import { ScriptVariableConfig } from '../script-variable-config.model';

export interface ComponentGroupConfig {
  style: string;
  template_variables: Record<string, string | ScriptVariableConfig>;
  template_commands: Record<string, string>;
  template: string;
  components: ComponentConfig[];
}
