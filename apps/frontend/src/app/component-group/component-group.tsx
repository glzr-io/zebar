import { createEffect, on, onCleanup, onMount } from 'solid-js';
import { insert } from 'solid-js/web';

import template from './component-group.njk?raw';
import { ComponentGroupConfig } from '~/shared/user-config';
import { parseTemplate } from '~/shared/template-parsing';
import { ClockComponent } from '~/components/clock/clock-component';

export interface ComponentGroupProps {
  id: string;
  config: ComponentGroupConfig;
}

export function ComponentGroup(props: ComponentGroupProps) {
  const tempId = `group-${Math.random().toString().slice(2)}`;
  let element = document.createElement('div');
  element.id = tempId;

  createEffect(
    on(
      () => [
        props.config.template_variables,
        props.config.template_commands,
        props.config.components,
      ],
      () => {
        const oldElement = document.getElementById(tempId)!;
        oldElement.innerHTML = '';
        const fdsa = parseTemplate(template, getBindings());
        insert(oldElement, () => fdsa);

        fdsa.parentElement?.replaceWith(fdsa);
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
