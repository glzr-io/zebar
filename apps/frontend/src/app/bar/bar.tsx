import { createEffect, on, onCleanup, onMount } from 'solid-js';

import defaultTemplate from './bar.njk?raw';
import { BarConfig } from '~/shared/user-config';
import { parseTemplate } from '~/shared/template-parsing';
import { ComponentGroup } from '~/component-group/component-group';
import { insertAndReplace } from '~/shared/utils';

export function Bar(props: { config: BarConfig }) {
  const element = document.createElement('div');
  element.id = props.config.id;

  createEffect(
    on(
      () => props.config,
      () => {
        const dispose = insertAndReplace(
          document.getElementById(props.config.id)!,
          () =>
            parseTemplate(
              props.config.template ?? defaultTemplate,
              getBindings(),
            ),
        );
        onCleanup(() => dispose());
      },
    ),
  );

  function getBindings() {
    return {
      strings: {
        root_props: `id="${props.config.id}" class="${props.config.class_name}"`,
      },
      components: {
        // TODO: Dynamically create based on 'group/*' keys available in config.
        'group.left': () => (
          <ComponentGroup config={props.config['group/left']} />
        ),
        'group.center': () => (
          <ComponentGroup config={props.config['group/center']} />
        ),
        'group.right': () => (
          <ComponentGroup config={props.config['group/right']} />
        ),
      },
    };
  }

  onMount(() => console.log('Bar mounted'));
  onCleanup(() => console.log('Bar cleanup'));

  return element;
}
