import { createMemo } from 'solid-js';

import defaultTemplate from './glazewm-component.njk?raw';
import { createTemplateElement } from '~/shared/template-parsing';
import { GlazeWMComponentConfig } from '~/shared/user-config';

export function GlazeWMComponent(props: { config: GlazeWMComponentConfig }) {
  const bindings = createMemo(() => {
    return {
      strings: {
        binding_mode: '',
        root_props: `id="${props.config.id}" class="${props.config.class_name}"`,
        workspaces: '',
      },
      functions: {
        focus_workspace: () => {},
      },
    };
  });

  return createTemplateElement({
    bindings,
    config: () => props.config,
    defaultTemplate: () => defaultTemplate,
  });
}
