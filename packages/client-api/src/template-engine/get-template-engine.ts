import { createStore, reconcile } from 'solid-js/store';

import { getRandomWithoutCollision, memoize } from '~/utils';
import { TemplateNode, parseTokens } from './token-parsing';
import { renderTemplateNodes } from './rendering';
import { tokenizeTemplate } from './tokenizing';

const [cache, setCache] = createStore<Record<string, TemplateNode[]>>({});

// Store map of available functions on the window. This makes them
// available at runtime.
const contextFunctions: Record<string, Function> = {};
window.__ZEBAR_FNS = contextFunctions;

export interface TemplateEngine {
  render: (template: string, context: Record<string, unknown>) => string;
  clearCache: () => void;
}

export function getTemplateEngine(): TemplateEngine {
  return {
    render,
    clearCache,
  };
}

function render(template: string, context: Record<string, unknown>) {
  if (cache[template]) {
    return renderTemplateNodes(cache[template], context, {
      transformExpression,
    });
  }

  // Tokenize and parse the template. Cache the result.
  const tokens = tokenizeTemplate(template);
  const parsed = parseTokens(tokens);
  setCache(template, parseTokens(tokens));

  return renderTemplateNodes(parsed, context, {
    transformExpression,
  });
}

function transformExpression(expression: unknown) {
  if (typeof expression !== 'function') {
    return expression;
  }

  // Wrap function (to handle anonymous functions) and modify how the
  // function gets stringified. This allows it to be called at runtime.
  const wrappedFunction = () => expression();
  const key = getRandomWithoutCollision(contextFunctions);
  wrappedFunction.toString = () => `__ZEBAR_FUNCTIONS__.${key}()`;

  // Store wrapped function in function map on window.
  contextFunctions[key] = expression;

  return wrappedFunction;
}

function clearCache() {
  setCache(reconcile({}));
}
