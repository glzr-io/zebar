import { renderString } from 'nunjucks';

import { TemplateBindings } from './template-bindings.model';
import { insertAndReplace } from '../utils';

export function parseTemplate(
  template: string,
  bindings: TemplateBindings = {},
): Element {
  // Compile string bindings with template engine.
  const compiledTemplate = parseTemplateStrings(
    template,
    bindings.strings ?? {},
    {
      bindingsToEscape: [
        ...Object.keys(bindings.functions ?? {}),
        ...Object.keys(bindings.components ?? {}),
      ],
    },
  );

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
    const tempId = `${componentName}-${Math.random().toString().slice(2)}`;
    const replacementDiv = `<div id="${tempId}"></div>`;

    element.innerHTML = element.innerHTML.replace(
      new RegExp(`{{\\s*${componentName}\\s*}}`),
      replacementDiv,
    );

    const mount = element.querySelector(`#${tempId}`)!;
    insertAndReplace(mount, component);
  }

  return element.firstChild as Element;
}

export interface ParseTemplateStringsOptions {
  bindingsToEscape?: string[];
}

/**
 * Nunjucks is used to evaluate strings in the template.
 */
function parseTemplateStrings(
  template: string,
  bindings: Record<string, string | boolean | number>,
  options: ParseTemplateStringsOptions = {},
): string {
  const { bindingsToEscape = [] } = options;

  // Need to somehow ignore bindings that shouldn't be compiled by Nunjucks.
  // Accomplish this by wrapping them in '{{ }}'.
  const escapedBindings = bindingsToEscape.reduce(
    (acc, binding) => ({
      ...acc,
      [binding]: `{{ ${binding} }}`,
    }),
    bindings,
  );

  const compiledTemplate = renderString(template, {
    ...bindings,
    ...escapedBindings,
  });

  return compiledTemplate;
}
