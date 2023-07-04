import { renderString } from 'nunjucks';
import { JSXElement } from 'solid-js';

export interface ParseTemplateOptions {
  bindings?: {
    strings?: Record<string, string | boolean | number>;
    functions?: Record<string, (...args: unknown[]) => unknown>;
    components?: Record<string, () => JSXElement>;
  };
}

export function parseTemplate(
  template: string,
  options: ParseTemplateOptions,
): HTMLElement {
  const { bindings = {} } = options;

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

  return element;
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
