import {
  TemplateNode,
  TokenType,
  TextNode,
  TemplateNodeType,
  InterpolationNode,
  IfStatementNode,
  IfBranch,
  ForStatementNode,
  SwitchStatementNode,
  CaseBranch,
  Token,
  ElseBranch,
  DefaultBranch,
} from '../types';

export function parseTokens(tokens: Token[]) {
  let cursor = 0;
  const nodes: TemplateNode[] = [];

  while (cursor < tokens.length) {
    const node = parseStandaloneToken(tokens[cursor]);
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
      case TokenType.CASE_STATEMENT:
        throw new Error(
          'Cannot use a case statement without a switch statement.',
        );
      case TokenType.DEFAULT_STATEMENT:
        throw new Error(
          'Cannot use a default statement without a switch statement.',
        );
      case TokenType.ELSE_IF_STATEMENT:
        throw new Error(
          'Cannot use an else if statement without an if statement.',
        );
      case TokenType.ELSE_STATEMENT:
        throw new Error(
          'Cannot use an else statement without an if statement.',
        );
      default:
        throw new Error(`Unknown token type '${token.type}'.`);
    }
  }

  function parseNestedTokens(): TemplateNode[] {
    const nodes: TemplateNode[] = [];
    let next = tokens[cursor + 1];

    while (
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

    while (expect(TokenType.CASE_STATEMENT)) {
      const expression = need(TokenType.EXPRESSION).substring;
      need(TokenType.OPEN_BLOCK);
      const children = parseNestedTokens();

      branches.push({ type: 'case', expression, children });
      need(TokenType.CLOSE_BLOCK);
    }

    if (expect(TokenType.DEFAULT_STATEMENT)) {
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
    const nextMatching = expect(tokenType);

    if (!nextMatching) {
      throw new Error(`Expected token type '${tokenType}'.`);
    }

    return nextMatching;
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
