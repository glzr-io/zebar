import {
  createEffect,
  createMemo,
  createSignal,
  on,
  onCleanup,
  onMount,
} from 'solid-js';
import { insert } from 'solid-js/web';

import template from './clock-component.njk?raw';
import { ClockComponentConfig } from '~/shared/user-config';
import { parseTemplate } from '~/shared/template-parsing';

export interface ClockComponentProps {
  id: string;
  config: ClockComponentConfig;
}

export function ClockComponent(props: ClockComponentProps) {
  const [date, setDate] = createSignal(new Date());

  const minutes = createMemo(() => date().getMinutes());
  const hours = createMemo(() => date().getHours());
  const interval = setInterval(() => setDate(new Date()), 1000);

  const tempId = `clock-${Math.random().toString().slice(2)}`;
  let element = document.createElement('div');
  element.id = tempId;

  createEffect(
    on(
      () => [
        props.config?.template_variables,
        props.config?.template_commands,
        minutes(),
        hours(),
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

  onMount(() => console.log('Clock mounted'));
  onCleanup(() => {
    console.log('Clock cleanup'); // Never gets called.
    clearInterval(interval);
  });

  function getBindings() {
    return {
      strings: {
        minutes: minutes(),
        hours: hours(),
        root_props: `id="${tempId}" data-root="true"`,
      },
      components: {},
    };
  }

  return element;
}
