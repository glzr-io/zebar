export interface ScannerMatch {
  substring: string;
  endIndex: number;
  startIndex: number;
}

/**
 * Utility for advancing through an input string via regex.
 */
export function createStringScanner(input: string) {
  let cursor = 0;
  let remainder = input;
  let latestMatch: ScannerMatch | null = null;

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

    return (latestMatch = {
      substring: input.substring(originalCursor, cursor),
      endIndex: cursor,
      startIndex: originalCursor,
    });
  }

  function scan(regex: RegExp) {
    return scanWithPredicate(regex, match => match.index === 0);
  }

  function scanUntil(regex: RegExp) {
    const match = scanWithPredicate(regex, () => true);

    if (match) {
      return match;
    }

    const originalCursor = cursor;
    const originalRemainder = remainder;
    remainder = '';
    cursor += remainder.length;

    return (latestMatch = {
      substring: originalRemainder,
      endIndex: cursor,
      startIndex: originalCursor,
    });
  }

  return {
    get cursor() {
      return cursor;
    },
    get remainder() {
      return remainder;
    },
    get latestMatch() {
      return latestMatch;
    },
    get isEmpty() {
      return remainder === '';
    },
    scan,
    scanUntil,
  };
}
