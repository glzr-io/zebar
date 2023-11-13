import { onCleanup } from 'solid-js';
import { createStore } from 'solid-js/store';

import { memoize } from '~/utils';
import { DateTimeProviderConfig } from '~/user-config';

export const createDateTimeProvider = memoize(
  (config: DateTimeProviderConfig) => {
    const [store, setStore] = createStore({
      now: 0,
      minutes: 0,
      hours: 0,
    });

    refresh();
    const interval = setInterval(() => refresh(), config.refresh_interval_ms);
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
      get now() {
        return store.now;
      },
      get minutes() {
        return store.minutes;
      },
      get hours() {
        return store.hours;
      },
    };
  },
);
