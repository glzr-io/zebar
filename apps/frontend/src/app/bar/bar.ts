import { createMemo } from 'solid-js';

import defaultTemplate from './bar.njk?raw';
import { createTemplateElement } from '~/shared/template-parsing';
import { BarConfig, GroupConfig } from '~/shared/user-config';
import { Group } from '~/group/group';

export function Bar(config: BarConfig): Element {
  const bindings = createMemo(() => ({
    components: {
      groups: getBarGroups,
    },
  }));

  // Create an array of `Group` components based on 'group/**' keys in config.
  function getBarGroups() {
    const groupConfigs = Object.entries(config)
      .filter(([key, value]) => key.startsWith('group') && !!value)
      .map(([_, value]) => value) as GroupConfig[];

    return groupConfigs.map(groupConfig => Group(groupConfig));
  }

  return createTemplateElement({
    bindings,
    config: () => config,
    defaultTemplate: () => defaultTemplate,
  });
}
