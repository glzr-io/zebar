import { createMemo } from 'solid-js';
import html from 'solid-js/html';
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

  const compiledHtml = createMemo(() => {
    return renderString(defaultTemplate, {
      id: props.id,
      components: props.config.components,
    });
  });

  return html`${compiledHtml()}`;
}
