import { runTemplateEngine } from './run-template-engine';
import { TemplateBindings } from './template-bindings.model';

export function parseTemplate(
  template: string,
  varia: TemplateBindings,
): Element {
  const element = document.createElement('div');

  // Compile variable + slot bindings with template engine.
  element.innerHTML = runTemplateEngine(template, bindings);

  return element;
}
