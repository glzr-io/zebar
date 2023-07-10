import { Type } from 'class-transformer';

import { ComponentConfig } from './component-config.model';
import { ScriptVariableConfig } from '../script-variable-config.model';
import { ComponentConfigBase } from './component-config-base.model';
import { ClockComponentConfig } from './components/clock-component-config.model';
import { CpuComponentConfig } from './components/cpu-component-config.model';
import { GlazeWMComponentConfig } from './components/glazewm-component-config.model';

export class ComponentGroupConfig {
  id: string;
  class_name: string;
  style: string;
  template_variables: Record<string, string | ScriptVariableConfig>;
  template_commands: Record<string, string>;
  template: string;

  @Type(() => ComponentConfigBase, {
    discriminator: {
      property: 'type',
      subTypes: [
        { value: ClockComponentConfig, name: 'clock' },
        { value: CpuComponentConfig, name: 'cpu' },
        { value: GlazeWMComponentConfig, name: 'glazewm' },
      ],
    },
  })
  components: ComponentConfig[];
}
