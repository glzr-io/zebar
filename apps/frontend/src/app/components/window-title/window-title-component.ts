import { createMemo } from 'solid-js';

import defaultTemplate from './window-title-component.njk?raw';
import { createTemplateElement } from '~/shared/template-parsing';
import { WindowTitleComponentConfig } from '~/shared/user-config';

// TODO: Implement `WindowTitleComponent`.
export function WindowTitleComponent(props: {
  config: WindowTitleComponentConfig;
}) {
  const bindings = createMemo(() => {
    return {
      variables: {
        window_title: '',
      },
    };
  });

  return createTemplateElement({
    bindings,
    config: () => props.config,
    defaultTemplate: () => defaultTemplate,
  });
}
