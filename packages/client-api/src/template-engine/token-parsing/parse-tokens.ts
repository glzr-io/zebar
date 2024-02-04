import { TemplateError } from '../shared';
import { type Token, TokenType } from '../tokenizing';
import type { ForStatementNode } from './for-statement-node';
import type {
  IfStatementNode,
  IfBranch,
  ElseBranch,
} from './if-statement-node.model';
import type { InterpolationNode } from './interpolation-node.model';
import type {
  SwitchStatementNode,
  CaseBranch,
  DefaultBranch,
} from './switch-statement-node.model';
import { TemplateNodeType } from './template-node-type.model';
import type { TemplateNode } from './template-node.model';
import type { TextNode } from './text-node.model';

export function parseTokens(tokens: Token[]) {
  let cursor = 0;
  const nodes: TemplateNode[] = [];
  console.log('tokens', tokens);

  while (cursor < tokens.length) {
    const node = parseStandaloneToken(tokens[cursor]!);
    nodes.push(node);
    cursor += 1;
  }

  function parseStandaloneToken(token: Token): TemplateNode {
    switch (token.type) {
      case TokenType.TEXT:
        return parseText(token);
      case TokenType.OPEN_INTERPOLATION:
        return parseInterpolation(token);
      case TokenType.IF_STATEMENT:
        return parseIfStatement(token);
      case TokenType.FOR_STATEMENT:
        return parseForStatement(token);
      case TokenType.SWITCH_STATEMENT:
        return parseSwitchStatement(token);
      case TokenType.SWITCH_CASE_STATEMENT:
        throw new TemplateError(
          'Cannot use @case without a @switch statement.',
          token.startIndex,
        );
      case TokenType.SWITCH_DEFAULT_STATEMENT:
        throw new TemplateError(
          'Cannot use @default without a @switch statement.',
          token.startIndex,
        );
      case TokenType.ELSE_IF_STATEMENT:
        throw new TemplateError(
          'Cannot use @elseif without an @if statement.',
          token.startIndex,
        );
      case TokenType.ELSE_STATEMENT:
        throw new TemplateError(
          'Cannot use @else without an @if statement.',
          token.startIndex,
        );
      default:
        throw new TemplateError(
          `Unknown token type '${token.type}'.`,
          token.startIndex,
        );
    }
  }

  function parseNestedTokens(): TemplateNode[] {
    const nodes: TemplateNode[] = [];
    let next = tokens[cursor + 1];

    while (
      // TODO: Add null check here for `next`.
      next.type === TokenType.TEXT ||
      next.type === TokenType.OPEN_INTERPOLATION ||
      next.type === TokenType.IF_STATEMENT ||
      next.type === TokenType.FOR_STATEMENT ||
      next.type === TokenType.SWITCH_STATEMENT
    ) {
      cursor += 1;
      const node = parseStandaloneToken(next);
      nodes.push(node);
      next = tokens[cursor + 1];
    }

    return nodes;
  }

  function parseText(token: Token): TextNode {
    return {
      type: TemplateNodeType.TEXT,
      text: token.substring,
    };
  }

  function parseInterpolation(_token: Token): InterpolationNode {
    const expression = need(TokenType.EXPRESSION).substring;
    need(TokenType.CLOSE_INTERPOLATION);

    return {
      type: TemplateNodeType.INTERPOLATION,
      expression,
    };
  }

  function parseIfStatement(_token: Token): IfStatementNode {
    const branches: (IfBranch | ElseBranch)[] = [];

    const expression = need(TokenType.EXPRESSION).substring;
    need(TokenType.OPEN_BLOCK);
    const children = parseNestedTokens();

    branches.push({ type: 'if', expression, children });
    need(TokenType.CLOSE_BLOCK);

    while (expect(TokenType.ELSE_IF_STATEMENT)) {
      const expression = need(TokenType.EXPRESSION).substring;
      need(TokenType.OPEN_BLOCK).substring;
      const children = parseNestedTokens();

      branches.push({ type: 'else if', expression, children });
      need(TokenType.CLOSE_BLOCK);
    }

    if (expect(TokenType.ELSE_STATEMENT)) {
      need(TokenType.OPEN_BLOCK);
      const children = parseNestedTokens();

      branches.push({ type: 'else', expression: null, children });
      need(TokenType.CLOSE_BLOCK);
    }

    return {
      type: TemplateNodeType.IF_STATEMENT,
      branches,
    };
  }

  function parseForStatement(_token: Token): ForStatementNode {
    const expression = need(TokenType.EXPRESSION).substring;
    need(TokenType.OPEN_BLOCK);

    const children = parseNestedTokens();
    need(TokenType.CLOSE_BLOCK);

    return {
      type: TemplateNodeType.FOR_STATEMENT,
      expression,
      children,
    };
  }

  function parseSwitchStatement(_token: Token): SwitchStatementNode {
    const expression = need(TokenType.EXPRESSION).substring;
    need(TokenType.OPEN_BLOCK);

    const branches: (CaseBranch | DefaultBranch)[] = [];

    while (expect(TokenType.SWITCH_CASE_STATEMENT)) {
      const expression = need(TokenType.EXPRESSION).substring;
      need(TokenType.OPEN_BLOCK);
      const children = parseNestedTokens();

      branches.push({ type: 'case', expression, children });
      need(TokenType.CLOSE_BLOCK);
    }

    if (expect(TokenType.SWITCH_DEFAULT_STATEMENT)) {
      need(TokenType.OPEN_BLOCK);
      const children = parseNestedTokens();

      branches.push({ type: 'default', children });
      need(TokenType.CLOSE_BLOCK);
    }

    need(TokenType.CLOSE_BLOCK);

    return {
      type: TemplateNodeType.SWITCH_STATEMENT,
      expression,
      branches,
    };
  }

  function need(tokenType: TokenType): Token {
    const nextOfType = expect(tokenType);

    if (!nextOfType) {
      throw new TemplateError(
        `Expected token type '${tokenType}'.`,
        tokens[cursor + 1].startIndex,
      );
    }

    return nextOfType;
  }

  function expect(tokenType: TokenType): Token | null {
    const next = tokens[cursor + 1];

    if (next.type !== tokenType) {
      return null;
    }

    cursor += 1;
    return next;
  }

  return nodes;
}
