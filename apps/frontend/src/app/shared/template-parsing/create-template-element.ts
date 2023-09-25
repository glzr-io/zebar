import { Accessor, createEffect, on, onCleanup, onMount } from 'solid-js';

import { useLogger } from '../logging';
import { mount } from '../utils/mount';
import { runTemplateEngine } from './run-template-engine';

export interface CreateTemplateElementArgs {
  id: Accessor<string>;
  className: Accessor<string>;
  variables: Accessor<Record<string, unknown>>;
  commands: Accessor<Record<string, (...args: unknown[]) => unknown>>;
  template: Accessor<string>;
  slots: Accessor<Record<string, string>>;
}

export function createTemplateElement(args: CreateTemplateElementArgs) {
  const logger = useLogger(`.${args.className()}#${args.id()}`);

  // Create element with ID.
  const element = document.createElement('div');
  element.id = args.id();

  createEffect(
    on(
      () => args.variables(),
      () => {
        console.log('in component', args.variables(), args);

        // Compile template with template engine.
        const newElement = createRootElement();
        newElement.innerHTML = runTemplateEngine(
          args.template(),
          args.slots(),
          completeAssign(args.variables(), args.commands()),
        );

        // TODO: Is it actually necessary to use `createRoot` around the mounted
        // elemented ? Is`onCleanup` called corrrectly when it's omitted?
        const oldElement = document.getElementById(args.id());
        const dispose = mount(oldElement, newElement);

        onCleanup(() => dispose());
      },
    ),
  );

  function createRootElement() {
    const element = document.createElement('div');
    element.id = args.id();
    element.className = args.className();
    return element;
  }

  function completeAssign(target: any, ...sources: any[]) {
    sources.forEach(source => {
      let descriptors = Object.keys(source).reduce((descriptors, key) => {
        descriptors[key] = Object.getOwnPropertyDescriptor(source, key);
        return descriptors;
      }, {} as any);
      // by default, Object.assign copies enumerable Symbols too
      Object.getOwnPropertySymbols(source).forEach(sym => {
        let descriptor = Object.getOwnPropertyDescriptor(source, sym);
        if (descriptor?.enumerable) {
          descriptors[sym] = descriptor;
        }
      });
      Object.defineProperties(target, descriptors);
    });
    return target;
  }

  onMount(() => logger.debug('Mounted'));
  onCleanup(() => logger.debug('Cleanup'));

  return element;
}
