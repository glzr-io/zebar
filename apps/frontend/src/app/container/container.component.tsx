import { createMemo } from 'solid-js';
import html from 'solid-js/html';

import { ComponentContainerConfig } from '~/shared/user-config/user-config.model';

export interface ContainerProps {
  id: string;
  config: ComponentContainerConfig;
}

export function Container(props: ContainerProps) {
  const defaultTemplate = `
    <div class="container" {{ root_props }}">
	    {% for component in components %}
        {{ component }}
      {% endfor %}
    </div>
  `;

  const compiledHtml = createMemo(() => {
    return handlebars.compile(defaultTemplate, {
      id: props.id,
      components: props.config.components,
    });
  });

  return html(compiledHtml());
}
