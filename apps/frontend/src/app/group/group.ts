import { createMemo } from 'solid-js';

import defaultTemplate from './group.njk?raw';
import { ClockComponent } from '~/components/clock/clock-component';
import { CpuComponent } from '~/components/cpu/cpu-component';
import { GlazeWMComponent } from '~/components/glazewm/glazewm-component';
import { WeatherComponent } from '~/components/weather/weather-component';
import { createTemplateElement } from '~/shared/template-parsing';
import { GroupConfig } from '~/shared/user-config';

export function Group(config: GroupConfig): Element {
  const bindings = createMemo(() => ({
    components: {
      components: getBarComponents,
    },
  }));

  function getBarComponents() {
    return config.components.map(componentConfig => {
      switch (componentConfig.type) {
        case 'clock':
          return ClockComponent(componentConfig);
        case 'cpu':
          return CpuComponent(componentConfig);
        case 'glazewm':
          return GlazeWMComponent(componentConfig);
        case 'weather':
          return WeatherComponent(componentConfig);
        default:
          throw new Error(
            `Unrecognized component type ${componentConfig.type}`,
          );
      }
    });
  }

  return createTemplateElement({
    bindings,
    config: () => config,
    defaultTemplate: () => defaultTemplate,
  });
}
