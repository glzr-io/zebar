/**
 * Create a pseudo-random ID.
 */
export function createUniqueId(): string {
  return `id-${Math.random().toString().slice(2)}`;
}
