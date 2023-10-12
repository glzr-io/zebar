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
import { TemplateError } from './template-error';

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
    const { elementName, iterable } = parseForExpression(node.expression);

    return iterable
      .map((el, index) => {
        // Push element name and index (optionally) to local context.
        context.local.push({
          [elementName]: el,
          ['index']: 0,
        });

        const result = visitAll(el);
        context.local.pop();

        return result;
      })
      .join('');
  }

  function parseForExpression(expression: string) {
    try {
      const [elementName, iterable] = expression.split(' of ');

      return {
        elementName,
        iterable: evalExpression(iterable) as any[],
      };
    } catch (e) {
      throw new TemplateError(
        "@for loop doesn't have a valid expression. Must be in the format '@for (item of items) { ... }'.",
        0,
      );
    }
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
