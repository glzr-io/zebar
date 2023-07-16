import { renderString } from 'nunjucks';

import { TemplateBindings } from './template-bindings.model';
import { createUniqueId, insertAndReplace } from '../utils';
import { getBindingRegex } from './get-binding-regex';

export function parseTemplate(
  template: string,
  bindings: TemplateBindings,
): Element {
  // Compile string bindings with template engine.
  const compiledTemplate = runTemplateEngine(template, bindings);

  const element = document.createElement('div');
  element.innerHTML = compiledTemplate;

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
 * Nunjucks is used to evaluate strings in the template.
 */
function runTemplateEngine(
  template: string,
  bindings: TemplateBindings,
): string {
  const {
    strings = {},
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

  const firstPass = renderString(escapedTemplate, { ...strings, slot: slots });

  // TODO: Refactor this so that it doesn't make 2 passes.
  const escapedTemplate2 = firstPass.replace(
    getBindingRegex(bindingsToEscape, 'g'),
    '{% raw %}$&{% endraw %}',
  );

  return renderString(escapedTemplate2, { ...strings });
}
