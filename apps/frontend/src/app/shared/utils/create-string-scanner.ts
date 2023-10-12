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

  // Set `latestMatch` and advance the cursor accordingly.
  function setLatestMatch(matchIndex: number, matchLength: number) {
    const originalCursor = cursor;
    remainder = remainder.substring(matchIndex + matchLength);
    cursor += matchIndex + matchLength;

    return (latestMatch = {
      substring: input.substring(originalCursor, cursor),
      endIndex: cursor,
      startIndex: originalCursor,
    });
  }

  // If the regex matches at the *current* cursor position, set latest match
  // and advance the cursor.
  function scan(regex: RegExp): ScannerMatch | null {
    const match = regex.exec(remainder);

    return match?.index !== 0
      ? null
      : setLatestMatch(match.index, match[0].length);
  }

  // If the regex matches at any of the remaining input, set latest match and
  // advance the cursor. If there are no matches, advance the cursor to end of
  // input.
  function scanUntil(regex: RegExp): ScannerMatch {
    const match = regex.exec(remainder);

    return match
      ? setLatestMatch(match.index, match[0].length)
      : setLatestMatch(0, remainder.length);
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
