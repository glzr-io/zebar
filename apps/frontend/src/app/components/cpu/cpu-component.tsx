import { createMemo } from 'solid-js';

import defaultTemplate from './cpu-component.njk?raw';
import { createTemplateElement } from '~/shared/template-parsing';
import { CpuComponentConfig } from '~/shared/user-config';

export function CpuComponent(config: CpuComponentConfig): Element {
  const bindings = createMemo(() => {
    return {
      variables: {
        cpu_usage: 0,
        cpu_temp: 0,
        cpu_frequency: 1000,
      },
    };
  });

  return createTemplateElement({
    bindings,
    config: () => config,
    defaultTemplate: () => defaultTemplate,
  });
}
