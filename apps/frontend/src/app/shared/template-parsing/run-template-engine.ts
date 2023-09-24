import { renderString } from 'nunjucks';

/**
 * Nunjucks is used to evaluate variable + slot bindings in the template.
 */
export function runTemplateEngine(
  template: string,
  slots: Record<string, string>,
  templateContext: Record<string, unknown>,
): string {
  const compiledSlots = Object.keys(slots).reduce<Record<string, string>>(
    (acc, slot) => {
      return {
        ...acc,
        [slot]: renderString(slots[slot], templateContext),
      };
    },
    {},
  );

  return renderString(template, {
    ...templateContext,
    slot: compiledSlots,
  });
}
