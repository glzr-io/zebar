import { createEffect, createMemo, onCleanup, onMount } from 'solid-js';
import { ElementContext, createLogger } from 'zebar';

export interface TemplateElementProps {
  context: ElementContext;
}

export function TemplateElement(props: TemplateElementProps) {
  const config = props.context.parsedConfig;
  const logger = createLogger(`#${config.id}`);

  // Create element with ID.
  const element = document.createElement('div');
  element.id = config.id;

  const template = createMemo(() => {
    switch (config.template) {
      // TODO
      case 'template.glazewm_workspaces':
        return '';
      case 'template.weather':
        return '';
      default:
        return config.template;
    }
  });

  // Get a map of slot bindings where the keys are slot names.
  // ie. 'slot' and 'slot/top' -> { default: '...', top: '...' }
  const slots = createMemo(() => {
    return Object.keys(config)
      .filter(key => key === 'slot' || key.startsWith('slot/'))
      .reduce((acc, key) => {
        const slotName = key.split('/')[1] ?? 'default';

        return {
          ...acc,
          [slotName]: config[key as 'slot' | `slot/${string}`],
        };
      }, {});
  });

  // Update the HTML element when the template changes.
  createEffect(() => {
    const newElement = createRootElement();
    newElement.innerHTML = template();

    const oldElement = document.getElementById(config.id);
    oldElement!.replaceWith(newElement);
  });

  onMount(() => logger.debug('Mounted'));
  onCleanup(() => logger.debug('Cleanup'));

  function createRootElement() {
    const element = document.createElement('div');
    element.id = config.id;
    element.className = config.class_name;
    return element;
  }

  return element;
}
