import { createComputed } from 'solid-js';
import { createStore } from 'solid-js/store';

export function createContextStore(config: unknown) {
  const [contextTree, setContextTree] = createStore({});

  createComputed(createContextTree);

  function createContextTree() {
    const barConfigs = [];
  }

  return {
    store: contextTree,
  };
}
