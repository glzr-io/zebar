import { createMemo } from 'solid-js';

import glazewmWorkspacesTemplate from './templates/glazewm-workspaces.template.njk?raw';
import weatherTemplate from './templates/weather.template.njk?raw';
import { ProviderNode } from '~/shared/providers';
import { useTemplateParser } from '~/shared/template-parsing';
import { ComponentConfig } from '~/shared/user-config';

export interface BarComponentProps {
  config: ComponentConfig;
  provider: ProviderNode;
}

export function BarComponent(props: BarComponentProps) {
  const templateParser = useTemplateParser();

  const template = createMemo(() => {
    switch (props.config.template) {
      case 'template.glazewm_workspaces':
        return glazewmWorkspacesTemplate;
      case 'template.weather':
        return weatherTemplate;
      default:
        return props.config.template;
    }
  });

  // Get a map of slot bindings where the keys are slot names.
  // ie. 'slot' and 'slot/top' -> { default: '...', top: '...' }
  const slots = createMemo(() => {
    return Object.keys(props.config)
      .filter(key => key === 'slot' || key.startsWith('slot/'))
      .reduce((acc, key) => {
        const slotName = key.split('/')[1] ?? 'default';

        return {
          ...acc,
          [slotName]: props.config[key as 'slot' | `slot/${string}`],
        };
      }, {});
  });

  return templateParser.createElement({
    id: () => props.config.id,
    className: () => props.config.class_name,
    provider: props.provider,
    template,
  });
}
