import { renderString } from 'nunjucks';

import { TemplateBindings } from './template-bindings.model';
import { createUniqueId, insertAndReplace } from '../utils';
import { getBindingRegex } from './get-binding-regex';

export function parseTemplate(
  template: string,
  bindings: TemplateBindings,
): Element {
  // Compile variable + slot bindings with template engine.
  const compiledTemplate = runTemplateEngine(template, bindings);

  const element = document.createElement('div');
  element.innerHTML = compiledTemplate;

  // TODO: Move to user config valdiation, rather than handling this here.
  if (!element.firstChild) {
    throw new Error(
      "Invalid 'template' in config. Template must have a child element.",
    );
  }

  // Get component bindings that are used in the template.
  const componentBindings = Object.entries(bindings.components ?? {}).filter(
    ([componentName]) => compiledTemplate.includes(componentName),
  );

  for (const [componentName, component] of componentBindings) {
    // Create a temporary div that will be mounted by the component.
    const tempId = createUniqueId();
    const replacementDiv = `<div id="${tempId}"></div>`;

    element.innerHTML = element.innerHTML.replace(
      getBindingRegex(componentName),
      replacementDiv,
    );

    const mount = element.querySelector(`#${tempId}`)!;
    insertAndReplace(mount, component);
  }

  return element.firstChild as Element;
}

/**
 * Nunjucks is used to evaluate variable + slot bindings in the template.
 */
function runTemplateEngine(
  template: string,
  bindings: TemplateBindings,
): string {
  const {
    variables = {},
    slots = {},
    functions = {},
    components = {},
  } = bindings;

  const bindingsToEscape = [
    ...Object.keys(functions),
    ...Object.keys(components),
  ];

  // Need to ignore bindings that shouldn't be compiled by Nunjucks. Accomplish
  // this by wrapping them in '{% raw %}' tag.
  const escapedTemplate = template.replace(
    getBindingRegex(bindingsToEscape, 'g'),
    '{% raw %}$&{% endraw %}',
  );

  const compiledSlots = Object.keys(slots).reduce<Record<string, string>>(
    (acc, slot) => {
      return {
        ...acc,
        [slot]: renderString(slots[slot], variables),
      };
    },
    {},
  );

  return renderString(escapedTemplate, {
    ...variables,
    slot: compiledSlots,
  });
}
