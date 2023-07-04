import { createEffect, createSignal, on } from 'solid-js';

import { ComponentGroupConfig } from '~/shared/user-config/user-config.model';
import template from './component-group.njk?raw';
import { diffAndMutate } from '~/shared/utils/diff-and-mutate';
import { parseTemplate } from '~/shared/utils/parse-template';

export interface ComponentGroupProps {
  id: string;
  config: ComponentGroupConfig;
}

export function ComponentGroup(props: ComponentGroupProps) {
  const [components, setComponents] = createSignal<number[]>([]);

  // Test whether updates are working.
  setInterval(() => {
    setComponents([Math.random(), Math.random(), Math.random()]);
  }, 1000);

  const element = getTemplate();

  function getTemplate() {
    return parseTemplate(template, {
      id: props.id,
      components: components(),
    });
  }

  createEffect(
    on(
      () => components(),
      () => {
        diffAndMutate(element, getTemplate());
      },
    ),
  );

  return element;
}
