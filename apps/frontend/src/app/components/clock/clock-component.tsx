import {
  createEffect,
  createMemo,
  createSignal,
  on,
  onCleanup,
} from 'solid-js';

import template from './clock-component.njk?raw';
import { ClockComponentConfig } from '~/shared/user-config/clock-component-config.model';
import { parseTemplate } from '~/shared/template-parsing/parse-template';
import { updateParsedTemplate } from '~/shared/template-parsing';

export interface ClockComponentProps {
  id: string;
  config: ClockComponentConfig;
}

export function ClockComponent(props: ClockComponentProps) {
  const [date, setDate] = createSignal(new Date());

  const minutes = createMemo(() => date().getMinutes());
  const hours = createMemo(() => date().getHours());
  const interval = setInterval(() => setDate(new Date()), 1000);

  const element = parseTemplate(template, { bindings: getBindings() });

  createEffect(
    on(
      () => [
        props.config?.template_variables,
        props.config?.template_commands,
        minutes(),
        hours(),
      ],
      () =>
        updateParsedTemplate(element, template, { bindings: getBindings() }),
    ),
  );

  onCleanup(() => {
    console.log('cleanup'); // Never gets called.
    clearInterval(interval);
  });

  function getBindings() {
    return {
      strings: {
        minutes: minutes(),
        hours: hours(),
      },
      components: {},
    };
  }

  return element;
}
