import { createStringScanner } from '~/shared/utils';
import { Token, TokenType } from '../types';
import { TemplateError } from './template-error';

export enum TokenizeStateType {
  DEFAULT,
  IN_STATEMENT_ARGS,
  IN_STATEMENT_BLOCK,
  IN_EXPRESSION,
}

export interface InExpressionState {
  type: TokenizeStateType.IN_EXPRESSION;
  token?: Token;
  closeSymbol: string;
  ignoreSymbol?: string;
}

export type TokenizeState = { type: TokenizeStateType } | InExpressionState;

export function tokenizeTemplate(template: string): Token[] {
  // Stack of tokenize states. Last element represents current state.
  const stateStack: TokenizeState[] = [{ type: TokenizeStateType.DEFAULT }];

  // Tokens within input template.
  const tokens: Token[] = [];

  // String scanner for advancing through input template.
  const scanner = createStringScanner(template);

  function pushToken(typeOrToken: TokenType | Token) {
    const token =
      typeof typeOrToken === 'object'
        ? typeOrToken
        : { type: typeOrToken, ...scanner.latestMatch! };

    if (!token.substring) {
      throw new TemplateError('Cannot push an empty token.', scanner.cursor);
    }

    tokens.push(token);
  }

  function pushState(typeOrState: TokenizeStateType | InExpressionState) {
    const state =
      typeof typeOrState === 'object' ? typeOrState : { type: typeOrState };

    stateStack.push(state);
  }

  while (!scanner.isEmpty) {
    // Get current tokenize state.
    const state = stateStack[stateStack.length - 1].type;

    switch (state) {
      case TokenizeStateType.DEFAULT:
        tokenizeDefault();
        break;
      case TokenizeStateType.IN_STATEMENT_ARGS:
        tokenizeStatementArgs();
        break;
      case TokenizeStateType.IN_STATEMENT_BLOCK:
        tokenizeStatementBlock();
        break;
      case TokenizeStateType.IN_EXPRESSION:
        tokenizeExpression();
        break;
    }
  }

  function tokenizeDefault() {
    if (scanner.scan(/@if/)) {
      pushToken(TokenType.IF_STATEMENT);
      pushState(TokenizeStateType.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/@else\s+if/)) {
      pushToken(TokenType.ELSE_IF_STATEMENT);
      pushState(TokenizeStateType.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/@else/)) {
      pushToken(TokenType.ELSE_STATEMENT);
      pushState(TokenizeStateType.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/@for/)) {
      pushToken(TokenType.FOR_STATEMENT);
      pushState(TokenizeStateType.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/@switch/)) {
      pushToken(TokenType.SWITCH_STATEMENT);
      pushState(TokenizeStateType.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/@case/)) {
      pushToken(TokenType.SWITCH_CASE_STATEMENT);
      pushState(TokenizeStateType.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/@default/)) {
      pushToken(TokenType.SWITCH_DEFAULT_STATEMENT);
      pushState(TokenizeStateType.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/{{/)) {
      pushToken(TokenType.OPEN_INTERPOLATION);
      pushState({ type: TokenizeStateType.IN_EXPRESSION, closeSymbol: '}}' });
    } else if (scanner.scanUntil(/.*?(?={{|@|})/)) {
      // Search until a close block, the start of a statement, or the start of
      // an interpolation tag.
      pushToken(TokenType.TEXT);
    } else {
      throw new TemplateError('No valid tokens found.', scanner.cursor);
    }
  }

  function tokenizeStatementArgs() {
    if (scanner.scan(/\s+/)) {
      // Ignore whitespace within statement args.
    } else if (scanner.scan(/{/)) {
      pushToken(TokenType.OPEN_BLOCK);
      stateStack.pop();
      pushState(TokenizeStateType.IN_STATEMENT_BLOCK);
    } else if (scanner.scan(/\(/)) {
      pushState({ type: TokenizeStateType.IN_EXPRESSION, closeSymbol: ')' });
    } else {
      throw new TemplateError('Missing closing {.', scanner.cursor);
    }
  }

  function tokenizeStatementBlock() {
    if (scanner.scan(/}/)) {
      pushToken(TokenType.CLOSE_BLOCK);
      stateStack.pop();
    } else {
      tokenizeDefault();
    }
  }

  function tokenizeExpression() {
    const state = stateStack[stateStack.length - 1] as InExpressionState;
    const { closeSymbol, ignoreSymbol, token } = state;

    if (scanner.scan(/\s+/)) {
      // Ignore whitespace within expression.
    } else if (scanner.scan(/.*?(?='|`|")/)) {
      // Match expression until a string character. Closing symbol should be
      // ignored if within a string.
      const { startIndex, endIndex, substring } = scanner.latestMatch!;

      // Set or clear ignore symbol to the current string character.
      const stringCharacter = substring[substring.length - 1];
      state.ignoreSymbol =
        state.ignoreSymbol === stringCharacter ? undefined : stringCharacter;

      // Build expression token.
      state.token = {
        type: TokenType.EXPRESSION,
        startIndex: state.token?.startIndex ?? startIndex,
        endIndex,
        substring: (state.token?.substring ?? '') + substring,
      };
    } else if (!state.ignoreSymbol && scanner.scan(new RegExp(closeSymbol))) {
      // TODO
      pushToken(token);
      stateStack.pop();
    } else if (scanner.scan(/\s+/)) {
    } else {
      throw new TemplateError(
        `Missing close symbol ${closeSymbol}.`,
        scanner.cursor,
      );
    }
  }

  return tokens;
}
