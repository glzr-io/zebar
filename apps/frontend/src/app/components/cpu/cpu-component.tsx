import { createMemo } from 'solid-js';

import defaultTemplate from './cpu-component.njk?raw';
import { createTemplateElement } from '~/shared/template-parsing';
import { CpuComponentConfig } from '~/shared/user-config';

export function CpuComponent(props: { config: CpuComponentConfig }) {
  const bindings = createMemo(() => {
    return {
      variables: {
        root_props: `id="${props.config.id}" class="${props.config.class_name}"`,
      },
    };
  });

  return createTemplateElement({
    bindings,
    config: () => props.config,
    defaultTemplate: () => defaultTemplate,
  });
}
