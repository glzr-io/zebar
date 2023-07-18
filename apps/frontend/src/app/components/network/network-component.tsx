import { createMemo } from 'solid-js';

import defaultTemplate from './network-component.njk?raw';
import { createTemplateElement } from '~/shared/template-parsing';
import { NetworkComponentConfig } from '~/shared/user-config';

// TODO: Implement `NetworkComponent`.
export function NetworkComponent(props: { config: NetworkComponentConfig }) {
  const bindings = createMemo(() => {
    return {
      variables: {},
    };
  });

  return createTemplateElement({
    bindings,
    config: () => props.config,
    defaultTemplate: () => defaultTemplate,
  });
}
