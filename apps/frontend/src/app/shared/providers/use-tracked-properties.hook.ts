import { createSignal } from 'solid-js';

import { ProviderConfig } from '../user-config';
import { memoize } from '../utils';

// TODO: How to track user-defined variables (and functions)?
// TODO: Should `VariableConfig` also be passed in?
export const useTrackedVariables = memoize((config: ProviderConfig) => {
  const [trackedVariables, setTrackedVariables] = createSignal([]);

  // Is it possible to just equality match on variable/provider config?

  return trackedVariables;
});
