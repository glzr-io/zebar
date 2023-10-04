import { Accessor, createEffect, onCleanup, onMount } from 'solid-js';

import { useLogger } from '../logging';

export interface CreateTemplateElementArgs {
  id: string;
  className: string;
  template: Accessor<string>;
}

export function createTemplateElement(args: CreateTemplateElementArgs) {
  const logger = useLogger(`.${args.className}#${args.id}`);

  // Create element with ID.
  const element = document.createElement('div');
  element.id = args.id;

  createEffect(() => {
    // Create HTML element with the given template.
    const newElement = createRootElement();
    newElement.innerHTML = args.template();

    const oldElement = document.getElementById(args.id);
    oldElement!.replaceWith(newElement);
  });

  function createRootElement() {
    const element = document.createElement('div');
    element.id = args.id;
    element.className = args.className;
    return element;
  }

  onMount(() => logger.debug('Mounted'));
  onCleanup(() => logger.debug('Cleanup'));

  return element;
}
