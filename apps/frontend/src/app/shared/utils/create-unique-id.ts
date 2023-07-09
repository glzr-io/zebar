/**
 * Create a pseudo-random ID.
 */
export function createUniqueId(): string {
  return `id-${Math.random().toString(36).slice(2, 9)}`;
}
