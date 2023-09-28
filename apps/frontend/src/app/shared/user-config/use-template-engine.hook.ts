import { createStore } from 'solid-js/store';
import { Liquid, Template } from 'liquidjs';

import { memoize } from '../utils';

export const useTemplateEngine = memoize(() => {
  const [cache, setCache] = createStore<Record<string, Template[]>>({});

  var engine = new Liquid({ jsTruthy: true });

  function compile(template: string, templateContext: Record<string, unknown>) {
    if (cache[template]) {
      return engine.renderSync(cache[template], templateContext);
    }

    // Parse and cache template with LiquidJS.
    var parsed = engine.parse(template);
    setCache(template, engine.parse(template));

    return engine.renderSync(parsed, templateContext);
  }

  return {
    compile,
  };
});
