import {
  createSignal,
  onCleanup,
  onMount,
  runWithOwner,
  type Owner,
} from 'solid-js';

import type { ElementContext } from '~/index';
import {
  getScriptManager,
  type CustomProviderConfig,
} from '~/user-config';
import { isEventTarget, type PickPartial } from '~/utils';

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
  const [state, setState] = createSignal<unknown>();
  const scriptManager = getScriptManager();

  if (config.start_fn_path)
    await scriptManager.callFn(
      config.start_fn_path,
      new Event('custom'),
      elementContext,
    );

  if ('emitter_fn_path' in config) {
    const eventTarget = await scriptManager.callFn(
      config.emitter_fn_path,
      new Event('custom'),
      elementContext,
    );

    if (!isEventTarget(eventTarget))
      throw new TypeError(`Emitter function must return an event target.`);

    const listener = (event: Event) => {
      if (event.type !== 'value') return;

      if (!('value' in event)) return;

      setState(event.value);
    };

    runWithOwner(owner, () => {
      onMount(() => eventTarget.addEventListener('value', listener));
      onCleanup(async () => {
        eventTarget.removeEventListener('value', listener);

        if (config.stop_fn_path)
          await scriptManager.callFn(
            config.stop_fn_path,
            new Event('custom'),
            elementContext,
          );
      });
    });
  } else {
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
  }

  return {
    get state() {
      return state();
    },
  };
}
