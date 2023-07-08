import { createEffect, on, onCleanup, onMount } from 'solid-js';

import template from './component-group.njk?raw';
import { ComponentGroupConfig } from '~/shared/user-config';
import { parseTemplate } from '~/shared/template-parsing';
import { ClockComponent } from '~/components/clock/clock-component';
import { insertAndReplace } from '~/shared/utils';

export interface ComponentGroupProps {
  id: string;
  config: ComponentGroupConfig;
}

export function ComponentGroup(props: ComponentGroupProps) {
  const tempId = `group-${Math.random().toString().slice(2)}`;
  const element = document.createElement('div');
  element.id = tempId;

  createEffect(
    on(
      () => [
        props.config.template_variables,
        props.config.template_commands,
        props.config.components,
      ],
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
        root_props: `id="${tempId}" data-root="true"`,
      },
      components: {
        components: () => (
          // TODO: Avoid harcoding component + turn into array.
          <ClockComponent id="aaa" config={props.config.components[0]} />
        ),
      },
    };
  }

  onMount(() => console.log('ComponentGroup mounted'));
  onCleanup(() => console.log('ComponentGroup cleanup'));

  return element;
}
