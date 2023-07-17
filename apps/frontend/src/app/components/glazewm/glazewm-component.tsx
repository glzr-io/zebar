import { createMemo } from 'solid-js';

import defaultTemplate from './glazewm-component.njk?raw';
import { createTemplateElement } from '~/shared/template-parsing';
import { GlazeWMComponentConfig } from '~/shared/user-config';

export function GlazeWMComponent(props: { config: GlazeWMComponentConfig }) {
  const bindings = createMemo(() => {
    return {
      variables: {
        binding_mode: '',
        workspaces: [
          { name: '1', state: 'focused' },
          { name: '2', state: 'active' },
          { name: '3', state: 'normal' },
          { name: '4', state: 'normal' },
        ],
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
