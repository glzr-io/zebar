import { createEffect, onCleanup, onMount } from 'solid-js';
import { type ElementContext, createLogger, toCssSelector } from 'zebar';

export interface TemplateElementProps {
  context: ElementContext;
}

export function TemplateElement(props: TemplateElementProps) {
  const config = props.context.parsedConfig;
  const logger = createLogger(`#${config.id}`);

  // Create element with ID.
  const element = createRootElement();

  // Update the HTML element when the template changes.
  // @ts-ignore - TODO
  createEffect(() => (element.innerHTML = config.template));

  onMount(() => logger.debug('Mounted'));
  onCleanup(() => logger.debug('Cleanup'));

  function createRootElement() {
    const element = document.createElement('div');
    element.id = toCssSelector(config.id);
    element.className = config.class_names.join(' ');
    return element;
  }

  return element;
}
