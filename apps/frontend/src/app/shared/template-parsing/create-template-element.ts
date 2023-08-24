import { Accessor, createEffect, on, onCleanup, onMount } from 'solid-js';

import { TemplateElementConfig } from '../user-config';
import { parseTemplate } from './parse-template';
import { TemplateBindings } from './template-bindings.model';
import { useLogger } from '../logging';
import { mount } from '../utils/mount';

export interface CreateTemplateElementArgs {
  config: Accessor<TemplateElementConfig>;
  bindings: Accessor<TemplateBindings>;
  defaultTemplate: Accessor<string>;
}

export function createTemplateElement(args: CreateTemplateElementArgs) {
  const logger = useLogger(`.${args.config().class_name}#${args.config().id}`);

  // Create element with ID.
  const element = document.createElement('div');
  element.id = args.config().id;

  createEffect(
    on(
      () => args.bindings(),
      bindings => {
        // Merge component-specific bindings with defaults (eg. slot bindings
        // and `root_props`).
        const mergedBindings = {
          ...bindings,
          variables: {
            ...(bindings.variables ?? {}),
            ...(args.config().template_variables ?? {}),
            root_props: getRootProps(),
          },
          functions: {
            ...(bindings.functions ?? {}),
            ...commandsToFunctions(args.config().template_commands ?? {}),
          },
          slots: getSlotBindings(),
        };

        const parsedTemplate = parseTemplate(
          args.config().template ?? args.defaultTemplate(),
          mergedBindings,
        );

        const mountEl = document.getElementById(args.config().id);
        const dispose = mount(mountEl, parsedTemplate);

        onCleanup(() => dispose());
      },
    ),
  );

  function getRootProps() {
    return `id="${args.config().id}" class="${args.config().class_name}"`;
  }

  // Gets a map of slot bindings where keys are slot names.
  function getSlotBindings() {
    return Object.keys(args.config())
      .filter(key => key === 'slot' || key.startsWith('slot/'))
      .reduce((acc, key) => {
        const slotName = key.split('/')[1] ?? 'default';

        return {
          ...acc,
          [slotName]: args.config()[key as keyof TemplateElementConfig],
        };
      }, {});
  }

  function commandsToFunctions(templateCommands: Record<string, string>) {
    // TODO
    return {};
  }

  onMount(() => logger.debug('Mounted'));
  onCleanup(() => logger.debug('Cleanup'));

  return element;
}
