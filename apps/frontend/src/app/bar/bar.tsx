import { createEffect, on, onCleanup, onMount } from 'solid-js';

import template from './bar.njk?raw';
import { BarConfig } from '~/shared/user-config';
import { parseTemplate } from '~/shared/template-parsing';
import { ComponentGroup } from '~/component-group/component-group';
import { insertAndReplace } from '~/shared/utils';

export interface BarProps {
  id: string;
  config: BarConfig;
}

export function Bar(props: BarProps) {
  const tempId = `bar-${Math.random().toString().slice(2)}`;
  const element = document.createElement('div');
  element.id = tempId;

  createEffect(
    on(
      () => props.config,
      () => {
        const dispose = insertAndReplace(document.getElementById(tempId)!, () =>
          parseTemplate(template, getBindings()),
        );
        onCleanup(() => dispose());
      },
    ),
  );

  function getBindings() {
    return {
      strings: {
        root_props: `id="${tempId}"`,
      },
      components: {
        // TODO: Dynamically create based on 'group/*' keys available in config.
        'group.left': () => (
          <ComponentGroup id="aaa" config={props.config['group/left']} />
        ),
        'group.center': () => (
          <ComponentGroup id="bbb" config={props.config['group/center']} />
        ),
        'group.right': () => (
          <ComponentGroup id="ccc" config={props.config['group/right']} />
        ),
      },
    };
  }

  onMount(() => console.log('Bar mounted'));
  onCleanup(() => console.log('Bar cleanup'));

  return element;
}
