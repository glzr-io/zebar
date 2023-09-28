import { Resource, createEffect, createRoot } from 'solid-js';
import { createStore } from 'solid-js/store';

import { UserConfig } from '../types/user-config.model';
import { getBarConfigs } from './get-bar-configs';
import { ProvidersConfigSchema } from '../types/bar/providers-config.model';
import { useProvider } from '~/shared/providers';
import { BaseElementConfig } from '../types/bar/base-element-config.model';
import { getGroupConfigs } from './get-group-configs';
import { formatConfigError } from './format-config-error';
import { useTemplateEngine } from '../use-template-engine.hook';

export interface ConfigStore {
  value: UserConfig | null;
}

export function createConfigStore(configObj: Resource<unknown>) {
  const templateEngine = useTemplateEngine();

  const [config, setConfig] = createStore<ConfigStore>({ value: null });

  createEffect(() => {
    if (!configObj()) {
      return;
    }

    let dispose: () => void;

    createRoot(dispose => {
      try {
        dispose = dispose;

        const parsedConfig = { ...(configObj() as UserConfig) };
        const barConfigs = getBarConfigs(configObj() as UserConfig);

        for (const barConfig of barConfigs) {
          const variables = getElementVariables(barConfig);
          const barConfigEntries = Object.entries(barConfig).map(
            ([key, value]) => {
              if (typeof value === 'string') {
                return [key, templateEngine.compile(value, variables)];
              } else {
                return [key, value];
              }
            },
          );

          // TODO: Make dynamic.
          console.log('variables', variables);
          console.log('barConfigEntries', barConfigEntries);
          console.log('barConfig', barConfig);

          parsedConfig['bar/default'] = Object.fromEntries(barConfigEntries);

          createEffect(() => {
            console.log('variables changed (bar)', variables);
          });

          for (const groupConfig of getGroupConfigs(barConfig)) {
            const variables = getElementVariables(groupConfig);

            createEffect(() => {
              console.log('variables changed (group)', variables);
            });

            for (const componentConfig of groupConfig.components ?? []) {
              const variables = getElementVariables(componentConfig);

              createEffect(() => {
                console.log('variables changed (component)', variables);
              });
            }
          }
        }

        console.log('parsedConfig', parsedConfig);
        setConfig({ value: parsedConfig });
      } catch (e) {
        dispose();
        throw formatConfigError(e);
      }
    });

    return () => dispose();
  });

  // TODO: Get variables from `variables` config as well.
  function getElementVariables(config: BaseElementConfig) {
    const providerConfigs = ProvidersConfigSchema.parse(
      config?.providers ?? [],
    );

    return providerConfigs.reduce(
      (acc, config) => ({
        ...acc,
        [config.type]: useProvider(config).variables,
      }),
      {},
    );
  }

  return config;
}
