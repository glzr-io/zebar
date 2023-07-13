import defaultTemplate from './bar.njk?raw';
import { createTemplateElement } from '~/shared/template-parsing';
import { BarConfig } from '~/shared/user-config';
import { ComponentGroup } from '~/component-group/component-group';

export function Bar(props: { config: BarConfig }) {
  function getBindings() {
    const groupNames = Object.keys(props.config)
      .filter(key => key.startsWith('group/'))
      .map(key => key.replace('group/', ''));

    // Dynamically create based on 'group/*' keys available in config.
    const groupComponentMap = groupNames.reduce(
      (acc, name) => ({
        ...acc,
        [`group.${name}`]: () => (
          <ComponentGroup config={props.config[`group/${name}`]} />
        ),
      }),
      {},
    );

    return {
      strings: {
        root_props: `id="${props.config.id}" class="${props.config.class_name}"`,
      },
      components: groupComponentMap,
    };
  }

  return createTemplateElement({
    bindings: getBindings,
    config: () => props.config,
    defaultTemplate: () => defaultTemplate,
  });
}
