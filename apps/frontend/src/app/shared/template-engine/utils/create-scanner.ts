export interface ScannerMatch {
  content: string;
  endIndex: number;
  startIndex: number;
}

export function createScanner(template: string) {
  // TODO: Keep track of head and previous?
  let cursor = 0;
  let isTerminated = false;
  let remainder = template;
  let matched: ScannerMatch | null = null;

  function scanWithPredicate(
    regex: RegExp,
    predicate: (match: RegExpExecArray) => boolean,
  ) {
    const match = regex.exec(remainder);

    if (!match || !predicate(match)) {
      return null;
    }

    // If there is a successful match, advance the cursor.
    const originalCursor = cursor;
    remainder = remainder.substring(match.index + match[0].length);
    cursor += match.index + match[0].length;

    matched = {
      content: template.substring(originalCursor, cursor),
      endIndex: cursor,
      startIndex: originalCursor,
    };

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
