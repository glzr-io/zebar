/**
 * Convert a string to a valid CSS selector.
 */
export function toCssSelector(input: string): string {
  // Replace non-alphanumeric characters with hyphens.
  const sanitizedInput = input.replace(/[^a-zA-Z0-9]/g, '-');

  // Ensure the selector doesn't start with a number.
  return /^\d/.test(sanitizedInput)
    ? `_${sanitizedInput}`
    : sanitizedInput;
}
