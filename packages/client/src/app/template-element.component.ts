import { createEffect, onCleanup, onMount } from 'solid-js';
import {
  type ElementContext,
  createLogger,
  toCssSelector,
  getScriptManager,
} from 'zebar';
import morphdom from 'morphdom';

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

  createEffect(() => {
    // Subsequent template updates after the initial render

    // Since templates do not include the root template element,
    // copy the existing one without its children.
    const templateRoot = element.cloneNode(false) as Element;

    // Insert the template into the cloned root element
    templateRoot.innerHTML = (config as any).template;

    try {
      // Reconcile the DOM with the updated template
      // @ts-ignore - TODO: fix config.template type
      morphdom(element, templateRoot, {
        // Don't morph fromNode or toNode, only their children
        childrenOnly: true,
      });
    } catch (error) {
      // TODO - add error handling for reconciliation here
      logger.error(
        `Failed to reconciliate ${props.context.id} template:`,
        error,
      );
    }

    updateEventListeners();
  });

  onMount(() => {
    logger.debug('Mounted');
    try {
      // Initial render, set innerHTML to the template
      // @ts-ignore - TODO: fix config.template type
      element.innerHTML = config.template;
    } catch (error) {
      logger.error(
        `Initial render of ${[props.context.id]} failed:`,
        error,
      );
    }
  });
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
          selectorElement.addEventListener(
            eventConfig.type,
            eventCallback,
          );

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
