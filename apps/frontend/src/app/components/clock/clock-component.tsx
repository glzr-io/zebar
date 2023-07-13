import { createMemo, createSignal, onCleanup } from 'solid-js';

import defaultTemplate from './clock-component.njk?raw';
import { ClockComponentConfig } from '~/shared/user-config';
import { createTemplateElement } from '~/shared/template-parsing';

export function ClockComponent(props: { config: ClockComponentConfig }) {
  const [date, setDate] = createSignal(new Date());

  const minutes = createMemo(() => date().getMinutes());
  const hours = createMemo(() => date().getHours());

  const interval = setInterval(() => setDate(new Date()), 1000);
  onCleanup(() => clearInterval(interval));

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

  return createTemplateElement({
    bindings: getBindings,
    config: () => props.config,
    defaultTemplate: () => defaultTemplate,
  });
}
