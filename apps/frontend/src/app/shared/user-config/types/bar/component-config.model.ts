import { ClockComponentConfig } from './components/clock-component-config.model';
import { CpuComponentConfig } from './components/cpu-component-config.model';
import { GlazeWMComponentConfig } from './components/glazewm-component-config.model';

export type ComponentConfig =
  | ClockComponentConfig
  | CpuComponentConfig
  | GlazeWMComponentConfig;
