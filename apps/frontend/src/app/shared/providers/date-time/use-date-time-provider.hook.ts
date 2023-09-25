import { createMemo, createSignal, onCleanup } from 'solid-js';

import { memoize } from '../../utils';
import { useLogger } from '../../logging';
import { DateTimeProviderConfig } from '../../user-config';
import { createStore } from 'solid-js/store';

export const useDateTimeProvider = memoize((config: DateTimeProviderConfig) => {
  const logger = useLogger('useDateTime');

  const [store, setStore] = createStore({
    now: 0,
    minutes: 0,
    hours: 0,
  });

  // const interval = setInterval(() => refresh(), config.refresh_interval_ms);
  const interval = setInterval(() => refresh(), 1000);
  onCleanup(() => {
    console.log('ran cleanup');

    clearInterval(interval);
  });

  function refresh() {
    console.log('ran refresh');

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
