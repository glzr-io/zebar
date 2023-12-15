import { createStore, reconcile } from 'solid-js/store';

import { TemplateNode, parseTokens } from './token-parsing';
import { renderTemplateNodes } from './rendering';
import { tokenizeTemplate } from './tokenizing';

const [cache, setCache] = createStore<Record<string, TemplateNode[]>>({});

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
    return renderTemplateNodes(cache[template], context);
  }

  // Tokenize and parse the template. Cache the result.
  const tokens = tokenizeTemplate(template);
  const parsed = parseTokens(tokens);
  setCache(template, parseTokens(tokens));

  return renderTemplateNodes(parsed, context);
}

function clearCache() {
  setCache(reconcile({}));
}
