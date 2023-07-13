import { createEffect, on, onCleanup, onMount } from 'solid-js';

import defaultTemplate from './component-group.njk?raw';
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

  function getComponentType(componentConfig: ComponentConfig) {
    switch (componentConfig.type) {
      case 'clock':
        return <ClockComponent config={componentConfig} />;
      case 'cpu':
        return <p>Not implemented.</p>;
      case 'glazewm':
        return <p>Not implemented.</p>;
    }
  }

  function getBindings() {
    return {
      strings: {
        root_props: `id="${props.config.id}" class="${props.config.class_name}"`,
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
