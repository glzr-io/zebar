import { createApiClient } from '@glzr/data-access';

import { makeProvider } from './make-provider';

export const [ApiClientProvider, ApiClientContext, useApiClient] =
  makeProvider(createApiClient);
