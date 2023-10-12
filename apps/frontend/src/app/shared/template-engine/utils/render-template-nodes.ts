import { evalWithContext } from '~/shared/utils';
import {
  ForStatementNode,
  IfStatementNode,
  InterpolationNode,
  SwitchStatementNode,
  TemplateNode,
  TemplateNodeType,
  TextNode,
} from '../types';

export interface RenderContext {
  global: Record<string, unknown>;
  local: Record<string, unknown>[];
}

/**
 * Takes an abstract syntax tree and renders it to a string.
 */
export function renderTemplateNodes(
  nodes: TemplateNode[],
  globalContext: Record<string, unknown>,
) {
  const context: RenderContext = {
    global: globalContext,
    local: [],
  };

  function visitAll(nodes: TemplateNode[]): string {
    return nodes.map(node => visitOne(node)).join('');
  }

  function visitOne(node: TemplateNode): string {
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
    return evalExpression(node.expression);
  }

  function visitIfStatementNode(node: IfStatementNode): string {
    for (const branch of node.branches) {
      const shouldVisit =
        branch.type === 'else' || Boolean(evalExpression(branch.expression));

      if (shouldVisit) {
        return visitAll(branch.children);
      }
    }

    return '';
  }

  function visitForStatementNode(node: ForStatementNode): string {
    throw new Error('Function not implemented.');
  }

  function visitSwitchStatementNode(node: SwitchStatementNode): string {
    const value = evalExpression(node.expression);

    for (const branch of node.branches) {
      const shouldVisit =
        branch.type === 'default' ||
        value === evalExpression(branch.expression);

      if (shouldVisit) {
        return visitAll(branch.children);
      }
    }

    return '';
  }

  function evalExpression(expression: string) {
    return evalWithContext(expression, context.global, ...context.local);
  }

  return visitAll(nodes);
}
