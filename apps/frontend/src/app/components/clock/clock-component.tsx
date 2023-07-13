import {
  createEffect,
  createMemo,
  createSignal,
  on,
  onCleanup,
  onMount,
} from 'solid-js';

import defaultTemplate from './clock-component.njk?raw';
import { ClockComponentConfig } from '~/shared/user-config';
import { parseTemplate } from '~/shared/template-parsing';
import { insertAndReplace } from '~/shared/utils';

export function ClockComponent(props: { config: ClockComponentConfig }) {
  const [date, setDate] = createSignal(new Date());

  const minutes = createMemo(() => date().getMinutes());
  const hours = createMemo(() => date().getHours());
  const interval = setInterval(() => setDate(new Date()), 1000);

  const element = document.createElement('div');
  element.id = props.config.id;

  createEffect(
    on(
      () => [props.config, minutes(), hours()],
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

  onMount(() => console.log('Clock mounted'));
  onCleanup(() => {
    console.log('Clock cleanup');
    clearInterval(interval);
  });

  function getBindings() {
    return {
      strings: {
        minutes: minutes(),
        hours: hours(),
        root_props: `id="${props.config.id}" class="${props.config.class_name}"`,
      },
      components: {},
    };
  }

  return element;
}
