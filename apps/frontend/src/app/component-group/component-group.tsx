import { createEffect, on, onCleanup } from 'solid-js';

import template from './component-group.njk?raw';
import { ComponentGroupConfig } from '~/shared/user-config/user-config.model';
import { updateParsedTemplate } from '~/shared/template-parsing/update-parsed-template';
import { parseTemplate } from '~/shared/template-parsing/parse-template';
import { ClockComponent } from '~/components/clock/clock-component';

export interface ComponentGroupProps {
  id: string;
  config: ComponentGroupConfig;
}

export function ComponentGroup(props: ComponentGroupProps) {
  const element = parseTemplate(template, { bindings: getBindings() });

  createEffect(
    on(
      () => [
        props.config?.template_variables,
        props.config?.template_commands,
        props.config?.components,
      ],
      () =>
        updateParsedTemplate(element, template, { bindings: getBindings() }),
    ),
  );

  function getBindings() {
    return {
      strings: {
        id: props.id,
      },
      components: {
        components: () => (
          // TODO: Avoid harcoding component + turn into array.
          <ClockComponent id="aaa" config={props.config.components[0]} />
        ),
      },
    };
  }

  onCleanup(() => {
    console.log('cleanup'); // Never gets called.
  });

  return element;
}
