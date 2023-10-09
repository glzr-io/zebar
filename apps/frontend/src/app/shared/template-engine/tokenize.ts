import { createScanner } from './utils/create-scanner';
import { TokenType } from './types/token-type.model';
import { Token } from './types/token.model';
import { TemplateError } from './utils/template-error';

export enum TokenizerState {
  DEFAULT,
  IN_OUTPUT,
  IN_TAG_START,
  IN_TAG_BLOCK,
}

export function tokenize(template: string): Token[] {
  // Stack of tokenizer states.
  const stateStack: TokenizerState[] = [TokenizerState.DEFAULT];
  const tokens: Token[] = [];
  const scanner = createScanner(template);

  while (!scanner.isTerminated()) {
    // Get current tokenizer state.
    const state = stateStack[stateStack.length - 1];

    switch (state) {
      case TokenizerState.DEFAULT: {
        if (scanner.scan(/@if/)) {
          tokens.push({
            type: TokenType.IF_TAG_START,
            index: scanner.getCursor(),
          });
          stateStack.push(TokenizerState.IN_TAG_START);
        } else if (scanner.scan(/@else\s+if/)) {
          tokens.push({
            type: TokenType.ELSE_IF_TAG_START,
            index: scanner.getCursor(),
          });
          stateStack.push(TokenizerState.IN_TAG_START);
        } else if (scanner.scan(/@else/)) {
          tokens.push({
            type: TokenType.ELSE_TAG_START,
            index: scanner.getCursor(),
          });
          stateStack.push(TokenizerState.IN_TAG_START);
        } else if (scanner.scan(/@for/)) {
          tokens.push({
            type: TokenType.FOR_TAG_START,
            index: scanner.getCursor(),
          });
          stateStack.push(TokenizerState.IN_TAG_START);
        } else if (scanner.scan(/@switch/)) {
          tokens.push({
            type: TokenType.SWITCH_TAG_START,
            index: scanner.getCursor(),
          });
          stateStack.push(TokenizerState.IN_TAG_START);
        } else if (scanner.scan(/@case/)) {
          tokens.push({
            type: TokenType.CASE_TAG_START,
            index: scanner.getCursor(),
          });
          stateStack.push(TokenizerState.IN_TAG_START);
        } else if (scanner.scan(/{{/)) {
          tokens.push({
            type: TokenType.OPEN_INTERPOLATION_TAG,
            index: scanner.getCursor(),
          });
          stateStack.push(TokenizerState.IN_OUTPUT);
        } else if (scanner.scanUntil(/.*?(?={{|@)/)) {
          // Search until the start of a tag or output.
          tokens.push({
            type: TokenType.TEXT,
            index: scanner.getCursor(),
          });
        } else {
          tokens.push({
            type: TokenType.TEXT,
            index: scanner.getCursor(),
          });
          scanner.terminate();
        }
        break;
      }
      case TokenizerState.IN_OUTPUT: {
        if (scanner.scan(/\s+/)) {
          // Ignore whitespace within output.
        } else if (scanner.scan(/}}/)) {
          tokens.push({
            type: TokenType.CLOSE_INTERPOLATION_TAG,
            index: scanner.getCursor(),
          });
          stateStack.pop();
        } else if (scanner.scan(/.*?(?=}})/)) {
          // Match expression until closing `}}`.
          tokens.push({
            type: TokenType.EXPRESSION,
            index: scanner.getCursor(),
          });
        } else {
          console.log('bad output', scanner, tokens, stateStack, template);
          throw new TemplateError('aa', scanner.getCursor());
        }
        break;
      }
      case TokenizerState.IN_TAG_START: {
        if (scanner.scan(/\s+/)) {
          // Ignore whitespace within tag start.
        } else if (scanner.scan(/{/)) {
          tokens.push({
            type: TokenType.OPEN_TAG_BLOCK,
            index: scanner.getCursor(),
          });
          stateStack.pop();
          stateStack.push(TokenizerState.IN_TAG_BLOCK);
        } else if (scanner.scan(/\([\w\-]+\)/)) {
          // TODO: Need to ignore nested parenthesis within tag start.
          tokens.push({
            type: TokenType.EXPRESSION,
            index: scanner.getCursor(),
          });
        } else {
          console.log('bad tag start', scanner, tokens, stateStack, template);
          throw new TemplateError('aa', scanner.getCursor());
        }
        break;
      }
    }
  }

  return tokens;
}
