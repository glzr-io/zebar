import { onCleanup } from 'solid-js';
import { createStore } from 'solid-js/store';

import { memoize } from '~/utils';
import {
  DateTimeProviderOptions,
  DateTimeProviderOptionsSchema,
} from '~/user-config';

const DEFAULT = DateTimeProviderOptionsSchema.parse({});

export const createDateTimeProvider = memoize(
  (options: DateTimeProviderOptions = DEFAULT) => {
    const [store, setStore] = createStore({
      now: 0,
      minutes: 0,
      hours: 0,
    });

    refresh();
    const interval = setInterval(() => refresh(), options.refresh_interval_ms);
    onCleanup(() => clearInterval(interval));

    function refresh() {
      const date = new Date();

      setStore({
        now: date.valueOf(),
        minutes: date.getMinutes(),
        hours: date.getHours(),
      });
    }

    return {
      variables: store,
      commands: {},
    };
  },
);
