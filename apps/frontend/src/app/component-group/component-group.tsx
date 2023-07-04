import { createEffect, on, onCleanup } from 'solid-js';

import template from './component-group.njk?raw';
import { ComponentGroupConfig } from '~/shared/user-config/user-config.model';
import { diffAndMutate } from '~/shared/utils/diff-and-mutate';
import { parseTemplate } from '~/shared/utils/parse-template';

export interface ComponentGroupProps {
  id: string;
  config: ComponentGroupConfig;
}

export function ComponentGroup(props: ComponentGroupProps) {
  const element = getTemplate();

  createEffect(
    on(
      () => [
        props.config.template_variables,
        props.config.template_commands,
        props.config.components,
      ],
      () => {
        diffAndMutate(element, getTemplate());
      },
    ),
  );

  onCleanup(() => {
    console.log('cleanup');
  });

  function getTemplate() {
    return parseTemplate(template, {
      bindings: {
        strings: {
          id: props.id,
        },
        components: {},
      },
    });
  }

  return element;
}
