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

  // Currently active event listeners.
  let listeners: { type: string; fn: (event: Event) => Promise<any> }[] =
    [];

  // Update the HTML element when the template changes.
  createEffect(() => {
    clearEventListeners();
    // @ts-ignore - TODO
    element.innerHTML = config.template;
    addEventListeners();
  });

  onMount(() => logger.debug('Mounted'));
  onCleanup(() => logger.debug('Cleanup'));

  function createRootElement() {
    const element = document.createElement('div');
    element.className = config.class_names.join(' ');
    element.id = toCssSelector(config.id);
    return element;
  }

  function clearEventListeners() {
    listeners.forEach(({ type, fn }) =>
      element.removeEventListener(type, fn),
    );

    listeners = [];
  }

  function addEventListeners() {
    config.events.forEach(eventConfig => {
      const callFn = (event: Event) =>
        scriptManager.callFn(eventConfig.fn_path, event, props.context);

      element.addEventListener(eventConfig.type, callFn);
      listeners.push({ type: eventConfig.type, fn: callFn });
    });
  }

  return element;
}
