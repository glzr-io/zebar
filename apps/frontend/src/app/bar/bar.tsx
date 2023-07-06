import { createEffect, on } from 'solid-js';

import template from './bar.njk?raw';
import { BarConfig } from '~/shared/user-config/user-config.model';
import { parseTemplate } from '~/shared/template-parsing/parse-template';
import { ComponentGroup } from '~/component-group/component-group';
import { updateParsedTemplate } from '~/shared/template-parsing';

export interface BarProps {
  id: string;
  config: BarConfig;
}

export function Bar(props: BarProps) {
  const element = parseTemplate(template, { bindings: getBindings() });

  createEffect(
    on(
      () => [
        props.config.template_variables,
        props.config.template_commands,
        props.config.components_left,
        props.config.components_center,
        props.config.components_right,
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
        left: () => (
          <ComponentGroup id="aaa" config={props.config.components_left} />
        ),
        center: () => (
          <ComponentGroup id="bbb" config={props.config.components_center} />
        ),
        right: () => (
          <ComponentGroup id="ccc" config={props.config.components_right} />
        ),
      },
    };
  }

  return element;
}
