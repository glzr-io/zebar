import type { Owner } from 'solid-js';

import type { LanguageProviderConfig } from '~/user-config';
import { createProviderListener } from '../create-provider-listener';

export interface LanguageVariables {
  language: string;
}

export async function createLanguageProvider(
  config: LanguageProviderConfig,
  owner: Owner,
) {
  const languageVariables = await createProviderListener<
    LanguageProviderConfig,
    LanguageVariables
  >(config, owner);

  return {
    get language() {
      return languageVariables().language;
    },
  };
}
