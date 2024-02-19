import type { Owner } from 'solid-js';

import type { KomorebiProviderConfig } from '~/user-config';
import { createProviderListener } from '../create-provider-listener';

export interface KomorebiVariables {
  // TODO
  workspaces: any[];
}

export async function createKomorebiProvider(
  config: KomorebiProviderConfig,
  owner: Owner,
) {
  const komorebiVariables = await createProviderListener<
    KomorebiProviderConfig,
    KomorebiVariables
  >(config, owner);

  return {
    get workspaces() {
      return komorebiVariables().workspaces;
    },
  };
}
