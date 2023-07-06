import { diffAndMutate } from './diff-and-mutate';
import { parseTemplate } from './parse-template';
import { TemplateBindings } from './template-bindings.model';

export function updateParsedTemplate(
  element: Element,
  template: string,
  bindings: TemplateBindings = {},
) {
  const newElement = parseTemplate(template, bindings);

  diffAndMutate(element, newElement);
}
