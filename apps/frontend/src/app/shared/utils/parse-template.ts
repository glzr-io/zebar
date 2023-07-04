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

  // Nunjucks is used to evaluate strings in the template.
  const compiledTemplate = renderString(template, bindings.strings ?? {});

  const element = document.createElement('div');
  element.innerHTML = compiledTemplate;

  return element;
}
