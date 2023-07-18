import { createMemo } from 'solid-js';

import defaultTemplate from './battery-component.njk?raw';
import { createTemplateElement } from '~/shared/template-parsing';
import { BatteryComponentConfig } from '~/shared/user-config';

// TODO: Implement `BatteryComponent`.
export function BatteryComponent(props: { config: BatteryComponentConfig }) {
  const bindings = createMemo(() => {
    return {
      variables: {
        battery_level: 0,
      },
    };
  });

  return createTemplateElement({
    bindings,
    config: () => props.config,
    defaultTemplate: () => defaultTemplate,
  });
}
