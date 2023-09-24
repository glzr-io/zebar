import { createMemo, createSignal, onCleanup } from 'solid-js';

import { memoize } from '../../utils';
import { useLogger } from '../../logging';
import { DateTimeProviderConfig } from '../../user-config';

export const useDateTimeProvider = memoize((config: DateTimeProviderConfig) => {
  const logger = useLogger('useDateTime');

  const [date, setDate] = createSignal(new Date());
  const minutes = createMemo(() => date().getMinutes());
  const hours = createMemo(() => date().getHours());

  const interval = setInterval(
    () => setDate(new Date()),
    config.refresh_interval_ms,
  );

  onCleanup(() => clearInterval(interval));

  return {
    date,
    minutes,
    hours,
  };
});
