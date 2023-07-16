import { createMemo } from 'solid-js';

import defaultTemplate from './bar.njk?raw';
import { createTemplateElement } from '~/shared/template-parsing';
import { BarConfig } from '~/shared/user-config';
import { Group } from '~/group/group';

export function Bar(props: { config: BarConfig }) {
  const bindings = createMemo(() => {
    const groupNames = Object.keys(props.config)
      .filter(key => key.startsWith('group/'))
      .map(key => key.replace('group/', ''));

    // Dynamically create based on 'group/*' keys available in config.
    const groupComponentMap = groupNames.reduce(
      (acc, name) => ({
        ...acc,
        [`group.${name}`]: () => (
          <Group config={props.config[`group/${name}`]} />
        ),
      }),
      {},
    );

    return {
      variables: {
        root_props: `id="${props.config.id}" class="${props.config.class_name}"`,
      },
      components: groupComponentMap,
    };
  });

  return createTemplateElement({
    bindings,
    config: () => props.config,
    defaultTemplate: () => defaultTemplate,
  });
}
