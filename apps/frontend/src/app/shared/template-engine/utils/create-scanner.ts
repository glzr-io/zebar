export function createScanner(template: string) {
  // TODO: Keep track of head and previous?
  let cursor = 0;
  let isTerminated = false;
  let remainder = template;
  let matched = '';

  function scanWithPredicate(
    regex: RegExp,
    predicate: (match: RegExpExecArray) => boolean,
  ) {
    const match = regex.exec(remainder);

    if (!match || !predicate(match)) {
      return '';
    }

    // If there is a successful match, advance the cursor.
    matched = match[0];
    remainder = remainder.substring(match.index + matched.length);
    cursor += match.index + matched.length;
    // remainder = remainder.substring(cursor + matched.length);
    // cursor += cursor + matched.length;
    console.log(
      'found match:',
      match,
      match.index,
      'moved to new index',
      cursor,
      remainder,
    );

    return matched;
  }

  function scan(regex: RegExp) {
    return scanWithPredicate(regex, match => match.index === 0);
  }

  function scanUntil(regex: RegExp) {
    return scanWithPredicate(regex, () => true);
  }

  function terminate() {
    isTerminated = true;
  }

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
