import { onCleanup } from 'solid-js';
import { createStore } from 'solid-js/store';

import { memoize } from '../../utils';
import { DateTimeProviderConfig } from '../../user-config';

export const useDateTimeProvider = memoize((config: DateTimeProviderConfig) => {
  const [store, setStore] = createStore({
    now: 0,
    minutes: 0,
    hours: 0,
  });

  const interval = setInterval(() => refresh(), 1000);
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
});
