/**
 * Get regular expression for matching bindings in a template.
 */
export function getBindingRegex(
  bindingNames: string | string[],
  flags?: string,
): RegExp {
  if (Array.isArray(bindingNames)) {
    return new RegExp(`{{\\s*(${bindingNames.join('|')})\\s*}}`, flags);
  }

  return new RegExp(`{{\\s*${bindingNames}\\s*}}`, flags);
}
