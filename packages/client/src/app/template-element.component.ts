import { createEffect, createMemo, onCleanup, onMount } from 'solid-js';
import { ElementContext, createLogger, toCssSelector } from 'zebar';

export interface TemplateElementProps {
  context: ElementContext;
}

export function TemplateElement(props: TemplateElementProps) {
  const config = props.context.parsedConfig;
  const logger = createLogger(`#${props.context.parsedConfig.id}`);

  // Create element with ID.
  const element = document.createElement('div');
  const idSelector = toCssSelector(props.context.parsedConfig.id);
  element.id = idSelector;

  // Get a map of slot bindings where the keys are slot names.
  // ie. 'slot' and 'slot/top' -> { default: '...', top: '...' }
  // TODO: Slots aren't implemented.
  const slots = createMemo(() => {
    return Object.keys(config)
      .filter(key => key === 'slot' || key.startsWith('slot/'))
      .reduce((acc, key) => {
        const slotName = key.split('/')[1] ?? 'default';

        return {
          ...acc,
          // @ts-ignore - TODO
          [slotName]: config[key as 'slot' | `slot/${string}`],
        };
      }, {});
  });

  // Update the HTML element when the template changes.
  createEffect(() => {
    const newElement = createRootElement();
    // @ts-ignore - TODO
    newElement.innerHTML = props.context.parsedConfig.template;

    const oldElement = document.getElementById(idSelector);
    oldElement!.replaceWith(newElement);
  });

  onMount(() => logger.debug('Mounted'));
  onCleanup(() => logger.debug('Cleanup'));

  function createRootElement() {
    const element = document.createElement('div');
    element.id = idSelector;
    element.className = props.context.parsedConfig.class_names.join(' ');
    return element;
  }

  return element;
}
