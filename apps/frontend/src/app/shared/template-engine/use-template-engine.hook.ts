import { createStore } from 'solid-js/store';

import { memoize } from '../utils';
import { tokenizeTemplate } from './utils/tokenize-template';
import { parseTokens } from './utils/parse-tokens';
import { renderTemplateNodes } from './utils/render-template-nodes';
import { TemplateNode } from './types/template-node.model';

export const useTemplateEngine = memoize(() => {
  const [cache, setCache] = createStore<Record<string, TemplateNode[]>>({});

  // TODO: Wrap in try-catch and format the error to clearly show the error
  // position.
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
