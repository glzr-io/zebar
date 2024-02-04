import { createStore, reconcile } from 'solid-js/store';

import { type TemplateNode, parseTokens } from './token-parsing';
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
  const cacheHit = cache[template];

  if (cacheHit) {
    return renderTemplateNodes(cacheHit, context);
  }

  // Tokenize and parse the template. Cache the result.
  const tokens = tokenizeTemplate(template);
  const parsed = parseTokens(tokens);
  setCache(template, parsed);

  return renderTemplateNodes(parsed, context);
}

function clearCache() {
  setCache(reconcile({}));
}
