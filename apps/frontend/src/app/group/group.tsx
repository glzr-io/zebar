import { createMemo } from 'solid-js';

import defaultTemplate from './group.njk?raw';
import { ClockComponent } from '~/components/clock/clock-component';
import { createTemplateElement } from '~/shared/template-parsing';
import { ComponentConfig, GroupConfig } from '~/shared/user-config';

export function Group(props: { config: GroupConfig }) {
  function getComponentType(componentConfig: ComponentConfig) {
    switch (componentConfig.type) {
      case 'clock':
        return <ClockComponent config={componentConfig} />;
      case 'cpu':
        return <p>Not implemented.</p>;
      case 'glazewm':
        return <p>Not implemented.</p>;
    }
  }

  const bindings = createMemo(() => {
    return {
      strings: {
        root_props: `id="${props.config.id}" class="${props.config.class_name}"`,
      },
      components: {
        components: () => props.config.components.map(getComponentType),
      },
    };
  });

  return createTemplateElement({
    bindings,
    config: () => props.config,
    defaultTemplate: () => defaultTemplate,
  });
}
