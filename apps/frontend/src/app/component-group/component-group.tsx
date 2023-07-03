import { createEffect, createSignal } from 'solid-js';
import { renderString } from 'nunjucks';

import { ComponentGroupConfig } from '~/shared/user-config/user-config.model';

export interface ComponentGroupProps {
  id: string;
  config: ComponentGroupConfig;
}

export function ComponentGroup(props: ComponentGroupProps) {
  const defaultTemplate = `
    <div class="group" {{ root_props }}">
	    {% for component in components %}
        {{ component }}
      {% endfor %}
    </div>
  `;

  const [components, setComponents] = createSignal<number[]>([]);

  // Test whether updates are working.
  setInterval(() => {
    setComponents([Math.random(), Math.random(), Math.random()]);
  }, 1000);

  const element = document.createElement('div');
  element.innerHTML = parseTemplate();

  function parseTemplate() {
    return renderString(defaultTemplate, {
      id: props.id,
      components: components(),
    });
  }

  createEffect(() => {
    // TODO: Use element.replaceWith()?
    console.log('reran effect', components());
    element.innerHTML = parseTemplate();
  });

  return element;
}
