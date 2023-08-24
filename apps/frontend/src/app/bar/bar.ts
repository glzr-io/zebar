import { createMemo } from 'solid-js';

import defaultTemplate from './bar.njk?raw';
import { createTemplateElement } from '~/shared/template-parsing';
import { BarConfig } from '~/shared/user-config';
import { Group } from '~/group/group';

export function Bar(config: BarConfig): Element {
  const bindings = createMemo(() => ({
    components: getGroupComponents(),
  }));

  // Create a map of components based on 'group/*' keys available in config.
  // Map looks like `{ 'group.left': () => <Group ... /> }`.
  function getGroupComponents() {
    const groupNames = Object.keys(config)
      .filter(key => key.startsWith('group/'))
      .map(key => key.replace('group/', ''));

    return groupNames.reduce<Record<string, () => Element>>(
      (acc, name) => ({
        ...acc,
        [`group.${name}`]: () => Group(config[`group/${name}`]),
      }),
      {},
    );
  }

  return createTemplateElement({
    bindings,
    config: () => config,
    defaultTemplate: () => defaultTemplate,
  });
}
