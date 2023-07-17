import { JSXElement, createMemo } from 'solid-js';

import defaultTemplate from './bar.njk?raw';
import { createTemplateElement } from '~/shared/template-parsing';
import { BarConfig } from '~/shared/user-config';
import { Group } from '~/group/group';

export function Bar(props: { config: BarConfig }) {
  const bindings = createMemo(() => ({
    components: getGroupComponents(),
  }));

  // Create a map of components based on 'group/*' keys available in config.
  // Map looks like `{ 'group.left': () => <Group ... /> }`.
  function getGroupComponents() {
    const groupNames = Object.keys(props.config)
      .filter(key => key.startsWith('group/'))
      .map(key => key.replace('group/', ''));

    return groupNames.reduce<Record<string, () => JSXElement>>(
      (acc, name) => ({
        ...acc,
        [`group.${name}`]: () => (
          <Group config={props.config[`group/${name}`]} />
        ),
      }),
      {},
    );
  }

  return createTemplateElement({
    bindings,
    config: () => props.config,
    defaultTemplate: () => defaultTemplate,
  });
}
