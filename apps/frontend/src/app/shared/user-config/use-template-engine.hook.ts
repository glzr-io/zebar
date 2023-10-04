import { Liquid, Template } from 'liquidjs';
import { createEffect, createSignal } from 'solid-js';
import { createStore } from 'solid-js/store';

import { createGetterProxy, memoize } from '../utils';

export const useTemplateEngine = memoize(() => {
  const [trackedProperties, setTrackedProperties] = createSignal<
    (string | symbol)[][]
  >([]);

  const [cache, setCache] = createStore<Record<string, Template[]>>({});

  var engine = new Liquid({ jsTruthy: true });

  function compile(template: string, context: Record<string, unknown>) {
    const contextProxy = createGetterProxy(context, a => {
      setTrackedProperties(e => [...e, a]);
    });

    if (cache[template]) {
      return engine.renderSync(cache[template], contextProxy);
    }

    // Parse and cache template with LiquidJS.
    const parsed = engine.parse(template);
    setCache(template, engine.parse(template));

    return engine.renderSync(parsed, contextProxy);
  }

  createEffect(() => {
    console.log('trackedProperties', trackedProperties());
  });

  return {
    compile,
  };
});
