import { createEffect, on } from 'solid-js';

import template from './bar.njk?raw';
import { BarConfig } from '~/shared/user-config/user-config.model';
import { diffAndMutate } from '~/shared/utils/diff-and-mutate';
import { parseTemplate } from '~/shared/utils/parse-template';
import { ComponentGroup } from '~/component-group/component-group';

export interface BarProps {
  id: string;
  config: BarConfig;
}

export function Bar(props: BarProps) {
  const element = getParsedTemplate();

  createEffect(
    on(
      () => [
        props.config.template_variables,
        props.config.template_commands,
        props.config.components_left,
        props.config.components_center,
        props.config.components_right,
      ],
      () => diffAndMutate(element, getParsedTemplate()),
    ),
  );

  function getParsedTemplate() {
    return parseTemplate(template, {
      bindings: {
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
      },
    });
  }

  return element;
}
