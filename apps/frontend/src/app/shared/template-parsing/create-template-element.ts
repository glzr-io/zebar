import { Accessor, createEffect, on, onCleanup, onMount } from 'solid-js';

import { TemplateElementConfig } from '../user-config';
import { insertAndReplace } from '../utils';
import { parseTemplate } from './parse-template';
import { TemplateBindings } from './template-bindings.model';
import { useLogger } from '../logging';

export interface CreateTemplateElementProps {
  config: Accessor<TemplateElementConfig>;
  bindings: Accessor<TemplateBindings>;
  defaultTemplate: Accessor<string>;
}

export function createTemplateElement(props: CreateTemplateElementProps) {
  const logger = useLogger(
    `.${props.config().class_name}#${props.config().id}`,
  );

  // Create element with ID.
  const element = document.createElement('div');
  element.id = props.config().id;

  createEffect(
    on(
      () => props.bindings(),
      bindings => {
        const mount = document.getElementById(props.config().id)!;

        // Merge component-specific bindings with defaults (eg. slot bindings
        // and `root_props`).
        const mergedBindings = {
          ...bindings,
          variables: {
            ...(bindings.variables ?? {}),
            ...(props.config().template_variables ?? {}),
            root_props: getRootProps(),
          },
          functions: {
            ...(bindings.functions ?? {}),
            ...commandsToFunctions(props.config().template_commands ?? {}),
          },
          slots: getSlotBindings(),
        };

        const dispose = insertAndReplace(mount, () =>
          parseTemplate(
            props.config().template ?? props.defaultTemplate(),
            mergedBindings,
          ),
        );

        onCleanup(() => dispose());
      },
    ),
  );

  function getRootProps() {
    return `id="${props.config().id}" class="${props.config().class_name}"`;
  }

  // Gets a map of slot bindings where keys are slot names.
  function getSlotBindings() {
    return Object.keys(props.config())
      .filter(key => key === 'slot' || key.startsWith('slot/'))
      .reduce((acc, key) => {
        const slotName = key.split('/')[1] ?? 'default';

        return {
          ...acc,
          [slotName]: props.config()[key as keyof TemplateElementConfig],
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
