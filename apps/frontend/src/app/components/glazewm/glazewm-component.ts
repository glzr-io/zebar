import { createMemo } from 'solid-js';

import defaultTemplate from './glazewm-component.njk?raw';
import { createTemplateElement } from '~/shared/template-parsing';
import { GlazeWMComponentConfig } from '~/shared/user-config';
import { useGlazeWmProvider } from '~/shared/providers';

export function GlazeWMComponent(config: GlazeWMComponentConfig): Element {
  const glazeWm = useGlazeWmProvider();

  const bindings = createMemo(() => {
    return {
      variables: {
        binding_mode: '',
        workspaces: glazeWm.workspaces() ?? [],
      },
      functions: {
        focus_workspace: () => {},
      },
    };
  });

  return createTemplateElement({
    bindings,
    config: () => config,
    defaultTemplate: () => defaultTemplate,
  });
}
