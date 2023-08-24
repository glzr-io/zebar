import { createMemo, createSignal, onCleanup } from 'solid-js';

import defaultTemplate from './clock-component.njk?raw';
import { ClockComponentConfig } from '~/shared/user-config';
import { createTemplateElement } from '~/shared/template-parsing';

export function ClockComponent(config: ClockComponentConfig): Element {
  const [date, setDate] = createSignal(new Date());

  const minutes = createMemo(() => date().getMinutes());
  const hours = createMemo(() => date().getHours());

  const interval = setInterval(() => setDate(new Date()), 1000);
  onCleanup(() => clearInterval(interval));

  const bindings = createMemo(() => ({
    variables: {
      minutes: minutes(),
      hours: hours(),
    },
  }));

  return createTemplateElement({
    bindings,
    config: () => config,
    defaultTemplate: () => defaultTemplate,
  });
}
