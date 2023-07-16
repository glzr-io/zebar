import { getBindingRegex } from './get-binding-regex';
import { runTemplateEngine } from './run-template-engine';
import { TemplateBindings } from './template-bindings.model';
import { createUniqueId, insertAndReplace } from '../utils';

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
