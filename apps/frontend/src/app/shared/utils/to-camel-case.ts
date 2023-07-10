import { isObject } from './is-object';

export function toCamelCase(obj: unknown): unknown {
  if (Array.isArray(obj)) {
    obj.map(item => toCamelCase(item));
  }

  if (isObject(obj)) {
    return Object.keys(obj).reduce(
      (acc, key) => ({
        ...acc,
        [snakeToCamelCase(key)]: toCamelCase(
          (obj as Record<string, unknown>)[key],
        ),
      }),
      {} as Record<string, unknown>,
    );
  }

  return obj;
}

function snakeToCamelCase(str: string): string {
  return str
    .toLowerCase()
    .replace(/([-_][a-z])/g, group =>
      group.toUpperCase().replace('-', '').replace('_', ''),
    );
}
