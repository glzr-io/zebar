import { Signal } from 'solid-js';
import { createStore, unwrap, reconcile } from 'solid-js/store';

export function createDeepSignal<T>(value: T): Signal<T> {
  const [store, setStore] = createStore({
    value,
  });

  return [
    () => store.value,
    (incomingValue: T) => {
      const unwrapped = unwrap(store.value);

      if (typeof incomingValue === 'function') {
        incomingValue = incomingValue(unwrapped);
      }

      setStore('value', reconcile(incomingValue));
      return store.value;
    },
  ] as Signal<T>;
}
