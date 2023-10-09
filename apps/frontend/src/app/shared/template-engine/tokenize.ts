import { createScanner } from './utils/create-scanner';
import { TokenType } from './types/token-type.model';
import { Token } from './types/token.model';
import { TemplateError } from './utils/template-error';

export enum TokenizerState {
  DEFAULT,
  IN_TAG_PARAMS,
  IN_TAG_BLOCK,
  IN_INTERPOLATION,
}

export function tokenize(template: string): Token[] {
  // Stack of tokenizer states.
  const stateStack: TokenizerState[] = [TokenizerState.DEFAULT];
  const tokens: Token[] = [];
  const scanner = createScanner(template);

  function pushToken(type: TokenType) {
    tokens.push({
      type,
      index: scanner.getCursor(),
    });
  }

  while (!scanner.isTerminated()) {
    // Get current tokenizer state.
    const state = stateStack[stateStack.length - 1];

    switch (state) {
      case TokenizerState.DEFAULT: {
        tokenizeDefault();
        break;
      }
      case TokenizerState.IN_TAG_PARAMS: {
        tokenizeTagParams();
        break;
      }
      case TokenizerState.IN_TAG_BLOCK: {
        tokenizeTagBlock();
        break;
      }
      case TokenizerState.IN_INTERPOLATION: {
        tokenizeInterpolation();
        break;
      }
    }
  }

  function tokenizeDefault() {
    if (scanner.scan(/@if/)) {
      pushToken(TokenType.IF_TAG);
      stateStack.push(TokenizerState.IN_TAG_PARAMS);
    } else if (scanner.scan(/@else\s+if/)) {
      pushToken(TokenType.ELSE_IF_TAG);
      stateStack.push(TokenizerState.IN_TAG_PARAMS);
    } else if (scanner.scan(/@else/)) {
      pushToken(TokenType.ELSE_TAG);
      stateStack.push(TokenizerState.IN_TAG_PARAMS);
    } else if (scanner.scan(/@for/)) {
      pushToken(TokenType.FOR_TAG);
      stateStack.push(TokenizerState.IN_TAG_PARAMS);
    } else if (scanner.scan(/@switch/)) {
      pushToken(TokenType.SWITCH_TAG);
      stateStack.push(TokenizerState.IN_TAG_PARAMS);
    } else if (scanner.scan(/@case/)) {
      pushToken(TokenType.CASE_TAG);
      stateStack.push(TokenizerState.IN_TAG_PARAMS);
    } else if (scanner.scan(/{{/)) {
      pushToken(TokenType.OPEN_INTERPOLATION_TAG);
      stateStack.push(TokenizerState.IN_INTERPOLATION);
    } else if (scanner.scanUntil(/.*?(?={{|@)/)) {
      // Search until the start of a tag or interpolation.
      pushToken(TokenType.TEXT);
    } else {
      pushToken(TokenType.TEXT);
      scanner.terminate();
    }
  }

  function tokenizeTagParams() {
    if (scanner.scan(/\s+/)) {
      // Ignore whitespace within tag params.
    } else if (scanner.scan(/{/)) {
      pushToken(TokenType.OPEN_TAG_BLOCK);
      stateStack.pop();
      stateStack.push(TokenizerState.IN_TAG_BLOCK);
    } else if (scanner.scan(/\((.*?)\)/)) {
      // TODO: Need to ignore nested parenthesis within tag params.
      pushToken(TokenType.EXPRESSION);
    } else {
      console.log('bad tag start', scanner, tokens, stateStack, template);
      throw new TemplateError('aa', scanner.getCursor());
    }
  }

  function tokenizeTagBlock() {
    if (scanner.scan(/}/)) {
      pushToken(TokenType.CLOSE_TAG_BLOCK);
      stateStack.pop();
    } else {
      tokenizeDefault();
    }
  }

  function tokenizeInterpolation() {
    if (scanner.scan(/\s+/)) {
      // Ignore whitespace within interpolation tag.
    } else if (scanner.scan(/}}/)) {
      pushToken(TokenType.CLOSE_INTERPOLATION_TAG);
      stateStack.pop();
    } else if (scanner.scan(/.*?(?=}})/)) {
      // Match expression until closing `}}`.
      pushToken(TokenType.EXPRESSION);
    } else {
      console.log('bad output', scanner, tokens, stateStack, template);
      throw new TemplateError('aa', scanner.getCursor());
    }
  }

  return tokens;
}
