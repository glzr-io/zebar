import { Accessor, createEffect, onCleanup, onMount } from 'solid-js';

import { useLogger } from '../logging';
import { runTemplateEngine } from './run-template-engine';
import { ProviderNode } from '../providers';

export interface CreateTemplateElementArgs {
  id: Accessor<string>;
  className: Accessor<string>;
  provider: ProviderNode;
  template: Accessor<string>;
}

export function createTemplateElement(args: CreateTemplateElementArgs) {
  const logger = useLogger(`.${args.className()}#${args.id()}`);

  // Create element with ID.
  const element = document.createElement('div');
  element.id = args.id();

  createEffect(() => {
    // Compile template with template engine.
    const newElement = createRootElement();
    newElement.innerHTML = runTemplateEngine(
      args.template(),
      args.provider.slots,
      args.provider.variables,
    );

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
