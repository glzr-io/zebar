import { createEffect, createMemo, onCleanup, onMount } from 'solid-js';

import { ComponentConfig, GroupConfig } from '~/shared/user-config';
import { glazewmWorkspacesTemplate, weatherTemplate } from '~/shared/templates';
import { useLogger } from '~/shared/logging';

export interface BarComponentProps {
  config: ComponentConfig;
  parentConfig: GroupConfig;
}

export function BarComponent(props: BarComponentProps) {
  const logger = useLogger(`#${props.config.id}`);

  // Create element with ID.
  const element = document.createElement('div');
  element.id = props.config.id;

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

  // Update the HTML element when the template changes.
  createEffect(() => {
    const newElement = createRootElement();
    newElement.innerHTML = template();

    const oldElement = document.getElementById(props.config.id);
    oldElement!.replaceWith(newElement);
  });

  onMount(() => logger.debug('Mounted'));
  onCleanup(() => logger.debug('Cleanup'));

  function createRootElement() {
    const element = document.createElement('div');
    element.id = props.config.id;
    element.className = props.config.class_name;
    return element;
  }

  return element;
}
