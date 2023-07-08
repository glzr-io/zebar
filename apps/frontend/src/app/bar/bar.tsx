import { createEffect, on, onCleanup, onMount } from 'solid-js';
import { insert } from 'solid-js/web';

import template from './bar.njk?raw';
import { BarConfig } from '~/shared/user-config';
import { parseTemplate } from '~/shared/template-parsing';
import { ComponentGroup } from '~/component-group/component-group';

export interface BarProps {
  id: string;
  config: BarConfig;
}

export function Bar(props: BarProps) {
  const tempId = `bar-${Math.random().toString().slice(2)}`;
  // const element = parseTemplate(template, getBindings());
  let element = document.createElement('div');
  element.id = tempId;
  // element.innerHTML = '';
  //  element = parseTemplate(template, getBindings())
  // element = parseTemplate(
  //   element,
  //   template,
  //   getBindings(),
  // ) as HTMLDivElement;

  createEffect(
    on(
      () => [
        props.config.template_variables,
        props.config.template_commands,
        props.config.components_left,
        props.config.components_center,
        props.config.components_right,
      ],
      () => {
        // const fdsa = parseTemplate(template, getBindings()) as HTMLDivElement;
        // const oldElement = document.getElementById(tempId)!;
        // oldElement.innerHTML = '';
        // const oldElement = document.querySelectorAll(`#${tempId}`)!;
        // console.log('oldElement', oldElement);

        // oldElement.innerHTML = '';
        // console.log('oldElement', oldElement.cloneNode(true), tempId, fdsa);
        // element.innerHTML = '';
        const oldElement = document.getElementById(tempId)!;
        oldElement.innerHTML = '';
        const fdsa = parseTemplate(template, getBindings());
        insert(oldElement, () => fdsa);

        fdsa.parentElement?.replaceWith(fdsa);
        // element = fdsa;
        // render(() => fdsa, oldElement);
        // element = parseTemplate(
        //   element,
        //   template,
        //   getBindings(),
        // ) as HTMLDivElement;
        // element = updateParsedTemplate(element, template, getBindings()),
      },
      // { defer: true },
    ),
  );

  function getBindings() {
    return {
      strings: {
        root_props: `id="${tempId}" data-root="true"`,
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

  onMount(() => console.log('Bar mounted'));
  onCleanup(() => console.log('Bar cleanup'));

  return element;
}
