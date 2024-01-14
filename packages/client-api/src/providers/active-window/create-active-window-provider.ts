import type { ActiveWindowProviderConfig } from '~/user-config';

// TODO: Implement provider.
export async function createActiveWindowProvider(
  _: ActiveWindowProviderConfig,
) {
  return {
    title: '',
  };
}
