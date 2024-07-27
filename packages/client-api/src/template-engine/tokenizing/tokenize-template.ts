import { createStringScanner } from '~/utils';
import type { Token } from './token.model';
import { TokenType } from './token-type.model';
import { TemplateError } from '../shared';

export enum TokenizeStateType {
  DEFAULT,
  IN_STATEMENT_ARGS,
  IN_STATEMENT_BLOCK,
  IN_INTERPOLATION,
  IN_EXPRESSION,
}

export interface InExpressionState {
  type: TokenizeStateType.IN_EXPRESSION;
  token?: Token;
  closeRegex: RegExp;
  activeWrappingSymbol: string | null;
}

export type TokenizeState =
  | { type: TokenizeStateType }
  | InExpressionState;

export function tokenizeTemplate(template: string): Token[] {
  // Stack of tokenize states. Last element represents current state.
  const stateStack: TokenizeState[] = [
    { type: TokenizeStateType.DEFAULT },
  ];

  // Tokens within input template.
  const tokens: Token[] = [];

  // String scanner for advancing through input template.
  const scanner = createStringScanner(template);

  function pushToken(typeOrToken: TokenType | Token) {
    const token =
      typeof typeOrToken === 'object'
        ? typeOrToken
        : { type: typeOrToken, ...scanner.latestMatch! };

    // Skip pushing empty tokens.
    if (token.substring) {
      tokens.push(token);
    }
  }

  // Push a tokenize state.
  function pushState(typeOrState: TokenizeStateType | TokenizeState) {
    const state =
      typeof typeOrState === 'object'
        ? typeOrState
        : { type: typeOrState };

    stateStack.push(state);
  }

  function updateLatestState(state: Partial<TokenizeState>) {
    if (!stateStack.length) {
      throw new TemplateError(
        'Could not update latest state while tokenizing template.',
        scanner.cursor,
      );
    }

    stateStack[stateStack.length - 1] = {
      ...stateStack[stateStack.length - 1]!,
      ...state,
    };
  }

  // Get current tokenize state.
  function getState() {
    return stateStack[stateStack.length - 1];
  }

  while (!scanner.isEmpty) {
    switch (getState()!.type) {
      case TokenizeStateType.DEFAULT:
        tokenizeDefault();
        break;
      case TokenizeStateType.IN_STATEMENT_ARGS:
        tokenizeStatementArgs();
        break;
      case TokenizeStateType.IN_STATEMENT_BLOCK:
        tokenizeStatementBlock();
        break;
      case TokenizeStateType.IN_INTERPOLATION:
        tokenizeInterpolation();
        break;
      case TokenizeStateType.IN_EXPRESSION:
        tokenizeExpression();
        break;
    }
  }

  function tokenizeDefault() {
    if (scanner.scan(/\s*@if/)) {
      pushToken(TokenType.IF_STATEMENT);
      pushState(TokenizeStateType.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/\s*@else\s+if/)) {
      pushToken(TokenType.ELSE_IF_STATEMENT);
      pushState(TokenizeStateType.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/\s*@else/)) {
      pushToken(TokenType.ELSE_STATEMENT);
      pushState(TokenizeStateType.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/\s*@for/)) {
      pushToken(TokenType.FOR_STATEMENT);
      pushState(TokenizeStateType.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/\s*@switch/)) {
      pushToken(TokenType.SWITCH_STATEMENT);
      pushState(TokenizeStateType.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/\s*@case/)) {
      pushToken(TokenType.SWITCH_CASE_STATEMENT);
      pushState(TokenizeStateType.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/\s*@default/)) {
      pushToken(TokenType.SWITCH_DEFAULT_STATEMENT);
      pushState(TokenizeStateType.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/(?:[\n\r\v\f\t]+\s*|){{/)) {
      // Ignore newlines before interpolation tag. Spaces are not ignored
      // for when the interpolation tag is mixed in with text tokens.
      pushToken(TokenType.OPEN_INTERPOLATION);
      pushState(TokenizeStateType.IN_INTERPOLATION);
    } else if (scanner.scanUntil(/[\S\s]+?(?={{|@|\})/)) {
      // Search until a close block, the start of a statement, or the start
      // of an interpolation tag. In a JavaScript regex, the `.` character
      // does not match newline characters, so we instead use [\S\s].
      pushToken({
        type: TokenType.TEXT,
        ...scanner.latestMatch!,
      });
    } else {
      throw new TemplateError('No valid tokens found.', scanner.cursor);
    }
  }

  function tokenizeStatementArgs() {
    if (scanner.scan(/\)?\s+/)) {
      // Ignore the closing parenthesis and any following whitespace.
    } else if (scanner.scan(/\(/)) {
      pushState({
        type: TokenizeStateType.IN_EXPRESSION,
        closeRegex: /[\S\s]+?(?=\))/,
        activeWrappingSymbol: null,
      });
    } else if (scanner.scan(/{\s*/)) {
      pushToken(TokenType.OPEN_BLOCK);
      stateStack.pop();
      pushState(TokenizeStateType.IN_STATEMENT_BLOCK);
    } else {
      throw new TemplateError(
        'Missing closing { after statement.',
        scanner.cursor,
      );
    }
  }

  function tokenizeStatementBlock() {
    if (scanner.scan(/}\s*/)) {
      pushToken(TokenType.CLOSE_BLOCK);
      stateStack.pop();
    } else {
      tokenizeDefault();
    }
  }

  function tokenizeInterpolation() {
    if (scanner.scan(/\s+/)) {
      // Ignore whitespace within interpolation tag.
    } else if (scanner.scan(/}}(?:[\n\r\v\f\t]+\s*|)/)) {
      pushToken(TokenType.CLOSE_INTERPOLATION);
      stateStack.pop();
    } else if (scanner.scan(/[\S\s]*?/)) {
      pushState({
        type: TokenizeStateType.IN_EXPRESSION,
        closeRegex: /[\S\s]+?(?=}})/,
        activeWrappingSymbol: null,
      });
    } else {
      throw new TemplateError(
        'Missing closing }} after expression.',
        scanner.cursor,
      );
    }
  }

  function tokenizeExpression() {
    const state = getState() as InExpressionState;

    if (scanner.scan(/\s+/)) {
      // Ignore whitespace within expression.
    } else if (scanner.scan(state.closeRegex)) {
      const { startIndex, endIndex, substring } = scanner.latestMatch!;

      // String scanner for finding wrapping symbols within the matched
      // substring. The closing symbol should be ignored if wrapped within an
      // unclosed string or parenthesis.
      const subScanner = createStringScanner(substring);
      let activeWrappingSymbol = state.activeWrappingSymbol;

      while (!subScanner.isEmpty) {
        const symbolMatch = subScanner.scan(/[\S\s]*?('|`|\(|\)|")/);

        if (!symbolMatch) {
          break;
        }

        // Get last character of scanned string (either (, ), ', ", or `).
        const foundSymbol = symbolMatch.substring.trimEnd().slice(-1);

        activeWrappingSymbol = getActiveWrappingSymbol(
          activeWrappingSymbol,
          foundSymbol,
        );
      }

      // If there's an active wrapping symbol, update the token created thus
      // far, and continue scanning.
      if (activeWrappingSymbol) {
        updateLatestState({
          activeWrappingSymbol,
          token: {
            type: TokenType.EXPRESSION,
            startIndex: state.token?.startIndex ?? startIndex,
            endIndex,
            substring: (state.token?.substring ?? '') + substring,
          },
        });
        return;
      }

      pushToken({
        type: TokenType.EXPRESSION,
        startIndex: state.token?.startIndex ?? startIndex,
        endIndex,
        substring: (state.token?.substring ?? '') + substring.trimEnd(),
      });

      stateStack.pop();
    } else {
      throw new TemplateError(
        'Missing close symbol after expression.',
        scanner.cursor,
      );
    }
  }

  function getActiveWrappingSymbol(
    current: string | null,
    matched: string,
  ) {
    const isOpeningSymbol = matched !== ')';

    // Set active wrapping symbol to the matched symbol.
    if (!current && isOpeningSymbol) {
      return matched;
    }

    const inverse = current === '(' ? ')' : current;

    // Otherwise, set/clear wrapping symbol depending on whether the inverse
    // symbol matches.
    return matched === inverse ? null : current;
  }

  // If state stack includes more than just the default state, then a
  // statement or expression has no closing tag.
  if (stateStack.length > 1) {
    throw new TemplateError(
      'Missing close symbol after statement or expression.',
      scanner.cursor,
    );
  }

  return tokens;
}
