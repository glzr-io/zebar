import { Token, TokenType } from '../types';
import { createStringScanner } from '../../utils';
import { TemplateError } from './template-error';

export enum TokenizeState {
  DEFAULT,
  IN_STATEMENT_ARGS,
  IN_STATEMENT_BLOCK,
  IN_INTERPOLATION,
}

export function tokenizeTemplate(template: string): Token[] {
  // Stack of tokenize states. Last element represents current state.
  const stateStack: TokenizeState[] = [TokenizeState.DEFAULT];

  // Tokens within input template.
  const tokens: Token[] = [];

  // String scanner for advancing through input template.
  const scanner = createStringScanner(template);

  function pushToken(type: TokenType) {
    const match = scanner.latestMatch;

    if (!match) {
      throw new TemplateError('Cannot push an empty token.', scanner.cursor);
    }

    tokens.push({ type, ...match });
  }

  while (!scanner.isEmpty) {
    // Get current tokenize state.
    const state = stateStack[stateStack.length - 1];

    switch (state) {
      case TokenizeState.DEFAULT:
        tokenizeDefault();
        break;
      case TokenizeState.IN_STATEMENT_ARGS:
        tokenizeStatementArgs();
        break;
      case TokenizeState.IN_STATEMENT_BLOCK:
        tokenizeStatementBlock();
        break;
      case TokenizeState.IN_INTERPOLATION:
        tokenizeInterpolation();
        break;
    }
  }

  function tokenizeDefault() {
    if (scanner.scan(/@if/)) {
      pushToken(TokenType.IF_STATEMENT);
      stateStack.push(TokenizeState.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/@else\s+if/)) {
      pushToken(TokenType.ELSE_IF_STATEMENT);
      stateStack.push(TokenizeState.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/@else/)) {
      pushToken(TokenType.ELSE_STATEMENT);
      stateStack.push(TokenizeState.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/@for/)) {
      pushToken(TokenType.FOR_STATEMENT);
      stateStack.push(TokenizeState.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/@switch/)) {
      pushToken(TokenType.SWITCH_STATEMENT);
      stateStack.push(TokenizeState.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/@case/)) {
      pushToken(TokenType.CASE_STATEMENT);
      stateStack.push(TokenizeState.IN_STATEMENT_ARGS);
    } else if (scanner.scan(/{{/)) {
      pushToken(TokenType.OPEN_INTERPOLATION);
      stateStack.push(TokenizeState.IN_INTERPOLATION);
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
      stateStack.push(TokenizeState.IN_STATEMENT_BLOCK);
    } else if (scanner.scan(/\((.*?)\)/)) {
      // TODO: Need to ignore nested parenthesis within statement args.
      pushToken(TokenType.EXPRESSION);
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

  function tokenizeInterpolation() {
    if (scanner.scan(/\s+/)) {
      // Ignore whitespace within interpolation tag.
    } else if (scanner.scan(/}}/)) {
      pushToken(TokenType.CLOSE_INTERPOLATION);
      stateStack.pop();
    } else if (scanner.scan(/.*?(?=\s*}})/)) {
      // Match expression until closing `}}`.
      pushToken(TokenType.EXPRESSION);
    } else {
      throw new TemplateError('Missing closing }}.', scanner.cursor);
    }
  }

  return tokens;
}
