import { createStore } from 'solid-js/store';

import { memoize } from '../utils';
import { TemplateNode } from './types';
import { renderTemplateNodes, tokenizeTemplate, parseTokens } from './utils';

export const useTemplateEngine = memoize(() => {
  const [cache, setCache] = createStore<Record<string, TemplateNode[]>>({});

  function render(template: string, context: Record<string, unknown>) {
    if (cache[template]) {
      return renderTemplateNodes(cache[template], context);
    }

    // Tokenize and parse the template. Cache the result.
    const tokens = tokenizeTemplate(template);
    const parsed = parseTokens(tokens);
    setCache(template, parseTokens(tokens));

    // TODO: Remove logs.
    console.log('tokenized', tokens, template);
    console.log('parsed', parsed, template);

    return renderTemplateNodes(parsed, context);
  }

  return {
    render,
  };
});
