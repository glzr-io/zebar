import { createMemo } from 'solid-js';

import defaultTemplate from './weather-component.njk?raw';
import { createTemplateElement } from '~/shared/template-parsing';
import { WeatherComponentConfig } from '~/shared/user-config';

export function WeatherComponent(props: { config: WeatherComponentConfig }) {
  const bindings = createMemo(() => {
    return {
      strings: {
        root_props: `id="${props.config.id}" class="${props.config.class_name}"`,
      },
    };
  });

  return createTemplateElement({
    bindings,
    config: () => props.config,
    defaultTemplate: () => defaultTemplate,
  });
}
