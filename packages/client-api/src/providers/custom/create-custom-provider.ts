import {
  createSignal,
  onCleanup,
  runWithOwner,
  type Owner,
} from 'solid-js';

import type { ElementContext } from '~/index';
import {
  getScriptManager,
  type CustomProviderConfig,
} from '~/user-config';
import type { PickPartial } from '~/utils';

export interface CustomState {
  state: unknown;
}

export async function createCustomProvider(
  config: CustomProviderConfig,
  elementContext: PickPartial<
    ElementContext,
    'parsedConfig' | 'providers'
  >,
  owner: Owner,
): Promise<CustomState> {
  const scriptManager = getScriptManager();

  if (config.start_fn_path)
    await scriptManager.callFn(
      config.start_fn_path,
      new Event('custom'),
      elementContext,
    );

  const [state, setState] = createSignal<unknown>();

  // run refresh fn first time to set initial state
  setState(
    await scriptManager.callFn(
      config.refresh_fn_path,
      new Event('custom'),
      elementContext,
    ),
  );

  // and then every refresh interval
  const interval = setInterval(
    async () =>
      setState(
        await scriptManager.callFn(
          config.refresh_fn_path,
          new Event('custom'),
          elementContext,
        ),
      ),
    config.refresh_interval,
  );

  runWithOwner(owner, () => {
    onCleanup(async () => {
      clearInterval(interval);

      if (config.stop_fn_path)
        await scriptManager.callFn(
          config.stop_fn_path,
          new Event('custom'),
          elementContext,
        );
    });
  });

  return {
    get state() {
      return state();
    },
  };
}
