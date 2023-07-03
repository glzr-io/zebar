import { createEffect, createSignal, on } from 'solid-js';
import { renderString } from 'nunjucks';

import { ComponentGroupConfig } from '~/shared/user-config/user-config.model';
import template from './component-group.njk?raw';

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

  const element = document.createElement('div');
  element.innerHTML = parseTemplate();

  function parseTemplate() {
    return renderString(template, {
      id: props.id,
      components: components(),
    });
  }

  createEffect(
    on(
      () => components(),
      () => {
        element.innerHTML = parseTemplate();
      },
    ),
  );

  return element;
}
