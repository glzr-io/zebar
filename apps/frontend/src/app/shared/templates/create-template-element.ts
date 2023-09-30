import { Accessor, createEffect, onCleanup, onMount } from 'solid-js';

import { useLogger } from '../logging';
import { useTemplateEngine } from '../user-config';

export interface CreateTemplateElementArgs {
  id: Accessor<string>;
  className: Accessor<string>;
  variables: Record<string, unknown>;
  template: Accessor<string>;
}

export function createTemplateElement(args: CreateTemplateElementArgs) {
  const templateEngine = useTemplateEngine();
  const logger = useLogger(`.${args.className()}#${args.id()}`);

  // Create element with ID.
  const element = document.createElement('div');
  element.id = args.id();

  createEffect(() => {
    // Create HTML element with the given template.
    const newElement = createRootElement();
    newElement.innerHTML = args.template();

    const oldElement = document.getElementById(args.id());
    oldElement!.replaceWith(newElement);
  });

  function createRootElement() {
    const element = document.createElement('div');
    element.id = args.id();
    element.className = args.className();
    return element;
  }

  onMount(() => logger.debug('Mounted'));
  onCleanup(() => logger.debug('Cleanup'));

  return element;
}
