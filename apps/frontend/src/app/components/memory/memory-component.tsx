import { createMemo } from 'solid-js';

import defaultTemplate from './memory-component.njk?raw';
import { createTemplateElement } from '~/shared/template-parsing';
import { MemoryComponentConfig } from '~/shared/user-config';

// TODO: Implement `MemoryComponent`.
export function MemoryComponent(props: { config: MemoryComponentConfig }) {
  const bindings = createMemo(() => {
    return {
      variables: {
        mem_usage: 0,
      },
    };
  });

  return createTemplateElement({
    bindings,
    config: () => props.config,
    defaultTemplate: () => defaultTemplate,
  });
}
