import { runWithOwner, type Owner, createEffect } from 'solid-js';

import type { LanguageProviderConfig } from '~/user-config';
import { createProviderListener } from '../create-provider-listener';
import { createStore } from 'solid-js/store';

export interface LanguageVariables {
  language: string;
}

export async function createLanguageProvider(
  config: LanguageProviderConfig,
  owner: Owner,
) {
  const providerListener = await createProviderListener<
    LanguageProviderConfig,
    LanguageVariables
  >(config, owner);

  const [languageVariables, setLanguageVariables] = createStore(
    await getVariables(),
  );

  runWithOwner(owner, () => {
    createEffect(async () => setLanguageVariables(await getVariables()));
  });

  async function getVariables() {
    const state = providerListener();
    return { language: state.language };
  }

  return {
    get language() {
      return languageVariables.language;
    },
  };
}
