import { createEffect, type Owner } from 'solid-js';

import type { KomorebiProviderConfig } from '~/user-config';
import { createProviderListener } from '../create-provider-listener';
import { createStore } from 'solid-js/store';

export interface KomorebiVariables {
  // TODO
  workspaces: any[];
}

export async function createKomorebiProvider(
  config: KomorebiProviderConfig,
  owner: Owner,
) {
  const providerListener = await createProviderListener<
    KomorebiProviderConfig,
    KomorebiVariables
  >(config, owner);

  const komorebiVariables = createStore({
    workspaces: [],
  });

  createEffect(() => {
    // @ts-ignore
    // const { monitors } = providerListener();
    const aaa = providerListener();
    console.log('incoming!!!', aaa);
    // const state = JSON.parse(monitors);
    // console.log('state', state);

    // const workspaces = state.workspaces;
  });

  return {
    // get workspaces() {
    //   return providerListener().workspaces;
    // },
  };
}
