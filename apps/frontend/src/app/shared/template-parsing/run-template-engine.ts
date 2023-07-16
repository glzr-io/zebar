import { renderString } from 'nunjucks';
import { TemplateBindings } from './template-bindings.model';
import { getBindingRegex } from './get-binding-regex';

/**
 * Nunjucks is used to evaluate variable + slot bindings in the template.
 */
export function runTemplateEngine(
  template: string,
  bindings: TemplateBindings,
): string {
  const {
    variables = {},
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

  const compiledSlots = Object.keys(slots).reduce<Record<string, string>>(
    (acc, slot) => {
      return {
        ...acc,
        [slot]: renderString(slots[slot], variables),
      };
    },
    {},
  );

  return renderString(escapedTemplate, {
    ...variables,
    slot: compiledSlots,
  });
}
