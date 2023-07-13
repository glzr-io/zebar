import { Accessor, createEffect, on, onCleanup, onMount } from 'solid-js';

import { Element } from '../user-config';
import { insertAndReplace } from '../utils';
import { parseTemplate } from './parse-template';
import { TemplateBindings } from './template-bindings.model';
import { useLogger } from '../logging';

export interface CreateTemplateElementArgs {
  // TODO: Rename to TemplateElementConfig.
  config: Accessor<Element>;
  bindings: Accessor<TemplateBindings>;
  defaultTemplate: Accessor<string>;
}

export function createTemplateElement(props: CreateTemplateElementArgs) {
  const logger = useLogger(props.config().class_name);

  const element = document.createElement('div');
  element.id = props.config().id;

  createEffect(
    on(
      () => props.bindings(),
      bindings => {
        const dispose = insertAndReplace(
          document.getElementById(props.config().id)!,
          () =>
            parseTemplate(
              props.config().template ?? props.defaultTemplate(),
              bindings,
            ),
        );

        onCleanup(() => dispose());
      },
    ),
  );

  onMount(() => logger.debug('Mounted'));
  onCleanup(() => logger.debug('Cleanup'));

  return element;
}
