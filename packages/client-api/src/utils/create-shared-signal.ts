import { type Signal, createSignal } from 'solid-js';

const cache: Record<string, Signal<any>> = {};

export function createSharedSignal<T>(key: string, value: T): Signal<T> {
  if (cache[key]) {
    return cache[key] as Signal<T>;
  }

  return (cache[key] = createSignal(value));
}
