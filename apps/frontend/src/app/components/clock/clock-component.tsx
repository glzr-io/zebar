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

  const bindings = createMemo(() => {
    return {
      strings: {
        get minutes() {
          return minutes();
        },
        get hours() {
          return hours();
        },
        root_props: `id="${props.config.id}" class="${props.config.class_name}"`,
      },
      components: {},
    };
  });

  return createTemplateElement({
    bindings,
    config: () => props.config,
    defaultTemplate: () => defaultTemplate,
  });
}
