import {
  createEffect,
  createMemo,
  createSignal,
  on,
  onCleanup,
} from 'solid-js';

import template from './clock-component.njk?raw';
import { ClockComponentConfig } from '~/shared/user-config/clock-component-config.model';
import { diffAndMutate } from '~/shared/utils/diff-and-mutate';
import { parseTemplate } from '~/shared/utils/parse-template';

export interface ClockComponentProps {
  id: string;
  config: ClockComponentConfig;
}

export function ClockComponent(props: ClockComponentProps) {
  const [date, setDate] = createSignal(new Date());

  const minutes = createMemo(() => date().getMinutes());
  const hours = createMemo(() => date().getHours());

  const element = getParsedTemplate();

  const interval = setInterval(() => setDate(new Date()), 1000);

  createEffect(
    on(
      () => [
        props.config?.template_variables,
        props.config?.template_commands,
        minutes(),
        hours(),
      ],
      () => diffAndMutate(element, getParsedTemplate()),
    ),
  );

  onCleanup(() => {
    console.log('cleanup'); // Never gets called.
    clearInterval(interval);
  });

  function getParsedTemplate() {
    return parseTemplate(template, {
      bindings: {
        strings: {
          minutes: String(minutes()),
          hours: String(hours()),
        },
        components: {},
      },
    });
  }

  return element;
}
