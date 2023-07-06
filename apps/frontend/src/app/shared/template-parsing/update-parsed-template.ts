import { diffAndMutate } from './diff-and-mutate';
import { parseTemplate } from './parse-template';
import { TemplateBindings } from './template-bindings.model';

export interface UpdateParsedTemplateOptions {
  bindings?: TemplateBindings;
}

export function updateParsedTemplate(
  element: Element,
  template: string,
  options: UpdateParsedTemplateOptions,
) {
  const { bindings = {} } = options;

  const newElement = parseTemplate(template, { bindings });

  diffAndMutate(element, newElement);
}
