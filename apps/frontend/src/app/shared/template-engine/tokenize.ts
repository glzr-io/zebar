import { TokenType } from './types/token-type.model';
import { Token } from './types/token.model';
import { TemplateError } from './utils/template-error';

export enum TokenizerState {
  DEFAULT,
  IN_START_TAG,
  IN_OUTPUT,
}

// const bindingRegex = /{{(.*?)}}/g.exec(template);

export function tokenize(template: string): Token[] {
  // Stack of
  const stateStack: TokenizerState[] = [TokenizerState.DEFAULT];
  const tokens: Token[] = [];
  const scanner: any = '';

  while (!scanner.isEmpty()) {
    switch (getState()) {
      case TokenizerState.DEFAULT: {
        if (scanner.scan(/aa/)) {
          //
        }
        break;
      }
      case TokenizerState.IN_OUTPUT: {
        if (scanner.scan(/\s+/)) {
          // Ignore whitespace within output.
        } else if (scanner.scan(/}}/)) {
          tokens.push({ index: scanner.index, type: TokenType.CLOSE_OUTPUT });
          stateStack.pop();
        } else if (scanner.scan(/[\w\-]+/)) {
          tokens.push({ index: scanner.index, type: TokenType.EXPRESSION });
        } else {
          throw new TemplateError('aa', scanner.index);
        }
        break;
      }
    }
  }

  // Get current tokenizer state.
  function getState(): TokenizerState {
    return stateStack[stateStack.length - 1];
  }

  return tokens;
}
