import { createEffect, on, onCleanup, onMount } from 'solid-js';

import template from './component-group.njk?raw';
import { ComponentConfig, ComponentGroupConfig } from '~/shared/user-config';
import { parseTemplate } from '~/shared/template-parsing';
import { ClockComponent } from '~/components/clock/clock-component';
import { insertAndReplace } from '~/shared/utils';

export function ComponentGroup(props: { config: ComponentGroupConfig }) {
  const element = document.createElement('div');
  element.id = props.config.id;

  createEffect(
    on(
      () => props.config,
      () => {
        const dispose = insertAndReplace(
          document.getElementById(props.config.id)!,
          () => parseTemplate(template, getBindings()),
        );
        onCleanup(() => dispose());
      },
    ),
  );

  function getComponentType(componentConfig: ComponentConfig) {
    switch (componentConfig.type) {
      case 'clock':
        return <ClockComponent config={componentConfig} />;
      case 'cpu':
        return <p>Not implemented.</p>;
      case 'glazewm':
        return <p>Not implemented.</p>;
      default:
        // TODO: This can probably be removed after adding class-validator.
        throw new Error(
          `Unknown component type '${
            (componentConfig as ComponentConfig).type
          }'.`,
        );
    }
  }

  function getBindings() {
    return {
      strings: {
        root_props: `id="${props.config.id}"`,
      },
      components: {
        components: () => props.config.components.map(getComponentType),
      },
    };
  }

  onMount(() => console.log('ComponentGroup mounted'));
  onCleanup(() => console.log('ComponentGroup cleanup'));

  return element;
}
