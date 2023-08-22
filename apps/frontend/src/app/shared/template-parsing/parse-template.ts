import { getBindingRegex } from './get-binding-regex';
import { runTemplateEngine } from './run-template-engine';
import { TemplateBindings } from './template-bindings.model';
import { createUniqueId, mount } from '../utils';

export function parseTemplate(
  template: string,
  bindings: TemplateBindings,
): Element {
  const element = document.createElement('div');

  // Compile variable + slot bindings with template engine.
  element.innerHTML = runTemplateEngine(template, bindings);

  // TODO: Move to user config valdiation, rather than handling this here.
  if (!element.firstChild) {
    throw new Error(
      "Invalid 'template' in config. Template must have a child element.",
    );
  }

  const componentBindings = Object.entries(bindings.components ?? {});

  for (const [componentName, component] of componentBindings) {
    // Create a temporary div that will be mounted by the component.
    const tempId = createUniqueId();
    const replacementDiv = `<div id="${tempId}"></div>`;

    element.innerHTML = element.innerHTML.replace(
      getBindingRegex(componentName),
      replacementDiv,
    );

    const mountEl = element.querySelector(`#${tempId}`);

    // Skip mounting if the component is not actually used in the template.
    if (mountEl) {
      mount(mountEl, component());
    }
  }

  return element.firstChild as Element;
}
