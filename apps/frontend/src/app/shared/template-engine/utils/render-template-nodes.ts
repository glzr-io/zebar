import {
  ForStatementNode,
  IfStatementNode,
  InterpolationNode,
  SwitchStatementNode,
  TemplateNode,
  TemplateNodeType,
  TextNode,
} from '../types';

/**
 * Takes an abstract syntax tree and renders it to a string.
 */
export function renderTemplateNodes(
  nodes: TemplateNode[],
  globalContext: Record<string, unknown>,
) {
  const context = {
    global: globalContext,
    local: {},
  };

  let cursor = 0;
  let output = '';

  while (cursor < nodes.length) {
    output += visit(nodes[cursor]);
    cursor += 1;
  }

  function visit(node: TemplateNode): string {
    switch (node.type) {
      case TemplateNodeType.TEXT:
        return visitTextNode(node);
      case TemplateNodeType.INTERPOLATION:
        return visitInterpolationNode(node);
      case TemplateNodeType.IF_STATEMENT:
        return visitIfStatementNode(node);
      case TemplateNodeType.FOR_STATEMENT:
        return visitForStatementNode(node);
      case TemplateNodeType.SWITCH_STATEMENT:
        return visitSwitchStatementNode(node);
    }
  }

  function visitTextNode(node: TextNode): string {
    return node.text;
  }

  function visitInterpolationNode(node: InterpolationNode): string {
    return evalWithContext(node.expression, globalContext);
  }

  function visitIfStatementNode(node: IfStatementNode): string {
    for (const branch of node.branches) {
      if (branch.type === 'else') {
        break;
      }

      if (Boolean(evalWithContext(branch.expression, globalContext))) {
        break;
      }
    }

    return '';
  }

  function visitForStatementNode(node: ForStatementNode): string {
    throw new Error('Function not implemented.');
  }

  function visitSwitchStatementNode(node: SwitchStatementNode): string {
    throw new Error('Function not implemented.');
  }

  function evalWithContext(
    expression: string,
    context: Record<string, unknown>,
  ) {
    // TODO: Pass context to eval.
    return eval(expression);
  }

  return output;
}
