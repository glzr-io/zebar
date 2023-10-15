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

/** Pattern for the expression in a for loop statement. */
const FOR_LOOP_EXPRESSION_PATTERN = /^\s*([(),\s0-9A-Za-z_$]*)\s+of\s+(.*)/;

/** Pattern for the loop variable on the left-side of a for loop expression. */
const FOR_LOOP_VARIABLE_PATTERN =
  /^\(?\s*([0-9A-Za-z_$]*)\s*,?\s*([0-9A-Za-z_$]*)/;

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
    console.log('nodes', nodes);

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
    const { loopVariable, indexVariable, iterable } = parseForExpression(
      node.expression,
    );

    return iterable
      .map((el, index) => {
        // Push loop variable and index (optionally) to local context.
        context.local.push({
          [loopVariable]: el,
          ...(indexVariable ? { [indexVariable]: index } : {}),
        });

        const result = visitAll(node.children);
        context.local.pop();

        return result;
      })
      .join('');
  }

  function parseForExpression(expression: string) {
    // try {
    const expressionMatch = expression.match(FOR_LOOP_EXPRESSION_PATTERN);
    const [_, loopVariableExpression, iterable] = expressionMatch ?? [];

    console.log('a', loopVariableExpression, iterable, expressionMatch);

    if (!loopVariableExpression || !iterable) {
      throw new Error();
    }

    console.log('b');
    const loopVariableMatch = loopVariableExpression.match(
      FOR_LOOP_VARIABLE_PATTERN,
    );
    const [__, loopVariable, indexVariable] = loopVariableMatch ?? [];

    if (!loopVariable) {
      throw new Error();
    }

    console.log('c', loopVariable, indexVariable);
    const x = {
      loopVariable,
      indexVariable,
      iterable: evalExpression(iterable) as any[],
    };
    console.log('d', x);
    return x;
    // } catch (e) {
    //   throw new TemplateError(
    //     "@for loop doesn't have a valid expression. Must be in the format '@for (item of items) { ... }'.",
    //     0,
    //   );
    // }
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
    console.log('evalExpression', expression);
    // const sum = new Function('context', `return (${expression})`);
    const evalFn = new Function(
      'global',
      'local',
      `with (global) { with (local) { return ${expression} } }`,
      // `with (global) { return ${expression} }`,
    );

    console.log('>', `with (global) { return ${expression} }`);
    console.log(
      '>>',
      context.local.reduce((acc, e) => ({ ...acc, ...e }), { people: ['bob'] }),
    );

    const res = evalFn(
      context.global,
      context.local.reduce((acc, e) => ({ ...acc, ...e }), { people: ['bob'] }),
    );
    console.log('res', res);

    return res;
    // return evalWithContext(expression, context.global, ...context.local);
  }

  return visitAll(nodes);
}
