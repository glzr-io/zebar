type ClassValue =
  | string
  | string[]
  | Record<string, boolean>
  | null
  | undefined;

/**
 * Utility for constructing `class` names conditionally.
 * Inspired by `clsx` https://github.com/lukeed/clsx.
 */
export function clsx(...inputs: ClassValue[]): string {
  let classString = '';

  for (const input of inputs) {
    if (input === null || input === undefined) {
      continue;
    }

    if (typeof input === 'string') {
      classString += `${input} `;
    }

    if (Array.isArray(input)) {
      input.forEach(inputPart => (classString += `${inputPart} `));
    }

    if (typeof input === 'object') {
      for (const [key, val] of Object.entries(input)) {
        if (!!val) {
          classString += `${key} `;
        }
      }
    }
  }

  return classString;
}
