import { createEffect, onCleanup, onMount } from 'solid-js';
import {
  type ElementContext,
  createLogger,
  toCssSelector,
  getScriptManager,
} from 'zebar';

export interface TemplateElementProps {
  context: ElementContext;
}

export function TemplateElement(props: TemplateElementProps) {
  const config = props.context.parsedConfig;
  const logger = createLogger(`#${config.id}`);
  const scriptManager = getScriptManager();

  // Create element with ID.
  const element = createRootElement();

  // Update the HTML element when the template changes.
  createEffect(() => {
    // @ts-ignore - TODO
    element.innerHTML = config.template;
    config.events.forEach(event => {
      element.addEventListener(event.type, () =>
        scriptManager.callFn(event.fn_path),
      );
    });
  });

  onMount(() => logger.debug('Mounted'));
  onCleanup(() => logger.debug('Cleanup'));

  function createRootElement() {
    const element = document.createElement('div');
    element.className = config.class_names.join(' ');
    element.id = toCssSelector(config.id);
    return element;
  }

  return element;
}
