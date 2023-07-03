import { renderString } from 'nunjucks';

export function parseTemplate(
  template: string,
  context: Record<string, unknown>,
): HTMLElement {
  const compiledTemplate = renderString(template, context);

  const element = document.createElement('div');
  element.innerHTML = compiledTemplate;

  return element;
}
