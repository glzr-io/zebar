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

interface ElementEventListener {
  eventType: string;
  eventCallback: (event: Event) => Promise<any>;
  selectorElement: Element;
}

export function TemplateElement(props: TemplateElementProps) {
  const config = props.context.parsedConfig;
  const logger = createLogger(`#${config.id}`);
  const scriptManager = getScriptManager();

  // Create element with ID.
  const element = createRootElement();

  // Currently active event listeners.
  let listeners: ElementEventListener[] = [];

  // Update the HTML element when the template changes.
  createEffect(() => {
    // @ts-ignore - TODO
    element.innerHTML = config.template;
    updateEventListeners();
  });

  onMount(() => logger.debug('Mounted'));
  onCleanup(() => logger.debug('Cleanup'));

  function createRootElement() {
    const element = document.createElement('div');
    element.className = config.class_names.join(' ');
    element.id = toCssSelector(config.id);
    return element;
  }

  function updateEventListeners() {
    // Remove existing event listeners.
    listeners.forEach(({ eventType, eventCallback, selectorElement }) =>
      selectorElement.removeEventListener(eventType, eventCallback),
    );

    listeners = [];

    config.events.forEach(eventConfig => {
      const eventCallback = (event: Event) =>
        scriptManager.callFn(eventConfig.fn_path, event, props.context);

      // Default to the root element if no selector is provided.
      const selectorElements = eventConfig.selector
        ? Array.from(element.querySelectorAll(eventConfig.selector))
        : [element];

      for (const selectorElement of selectorElements) {
        if (selectorElement) {
          selectorElement.addEventListener(eventConfig.type, eventCallback);

          listeners.push({
            eventType: eventConfig.type,
            eventCallback,
            selectorElement,
          });
        }
      }
    });
  }

  return element;
}
