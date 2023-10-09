export function createScanner(template: string) {
  // TODO: Keep track of head and previous?
  let cursor = 0;
  let isTerminated = false;
  let remainder = template;
  let matched = '';

  function scan(regex: RegExp) {
    const match = regex.exec(remainder);

    if (match?.index !== 0) {
      return '';
    }

    // If there is a match, advance the cursor to the end of the match.
    matched = match[0];
    remainder = remainder.substring(matched.length);
    cursor += matched.length;

    return matched;
  }

  function scanUntil(regex: RegExp) {
    const match = regex.exec(remainder);

    if (!match) {
      return '';
    }

    // If there is a match, advance the cursor to the end of the match.
    matched = match[0];
    remainder = remainder.substring(matched.length);
    cursor += matched.length;

    return matched;
  }

  function terminate() {
    isTerminated = true;
  }

  // TODO: Could simplify with `scanWithPredicate(e => e.index !== 0)`.
  return {
    getCursor: () => cursor,
    getRemainder: () => remainder,
    getMatched: () => matched,
    isTerminated: () => isTerminated,
    scan,
    scanUntil,
    terminate,
  };
}
