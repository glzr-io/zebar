import { Resource } from 'solid-js';
import { createStore } from 'solid-js/store';

import { UserConfig } from '../types/user-config.model';

export interface ConfigStore {
  value: UserConfig | null;
}

export function createConfigStore(configObj: Resource<unknown>) {
  const [config, setConfig] = createStore<ConfigStore>({ value: null });

  return config;
}
