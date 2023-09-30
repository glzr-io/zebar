import { Resource, createComputed, createEffect, createRoot } from 'solid-js';
import { createStore } from 'solid-js/store';

import { UserConfig, UserConfigP1Schema } from '../types/user-config.model';
import { ProvidersConfigSchema } from '../types/bar/providers-config.model';
import { useProvider } from '~/shared/providers';
import { BaseElementConfig } from '../types/bar/base-element-config.model';
import { formatConfigError } from './format-config-error';
import { useTemplateEngine } from '../use-template-engine.hook';
import { BarConfig, BarConfigSchemaP1 } from '../types/bar/bar-config.model';
import {
  GroupConfig,
  GroupConfigSchemaP1,
} from '../types/bar/group-config.model';
import { ComponentConfigSchemaP1 } from '../types/bar/component-config.model';
import { useConfigVariables } from '../use-config-variables.hook';

export interface ConfigStore {
  value: UserConfig | null;
}

export function createConfigStore(configObj: Resource<unknown>) {
  const templateEngine = useTemplateEngine();
  const configVariables = useConfigVariables();

  const [config, setConfig] = createStore<ConfigStore>({
    value: null,
  });

  createEffect(() => {
    if (!configObj() || !configVariables()) {
      return;
    }

    let dispose: () => void;

    createRoot(dispose => {
      dispose = dispose;

      try {
        const rootVariables = {
          env: configVariables()!,
        };

        setConfig('value', UserConfigP1Schema.parse(configObj()));

        const barConfigs = getBarConfigs(configObj() as UserConfig);

        // Traverse down bar config and update config with compiled +
        // validated properties.
        for (const [barKey, barConfig] of barConfigs) {
          const variables = getElementVariables(barConfig);
          const barId = `bar-${barKey.split('/')[1]}`;

          createComputed(() => {
            const parsedConfig = parseConfig(
              { ...barConfig, id: barId },
              BarConfigSchemaP1,
              variables,
            );

            setConfig('value', barKey, parsedConfig);
          });

          for (const [groupKey, groupConfig] of getGroupConfigs(barConfig)) {
            const variables = getElementVariables(groupConfig);
            const groupId = `${barId}-${groupKey.split('/')[1]}`;
            const componentConfigs = groupConfig.components ?? [];

            createComputed(() => {
              const parsedConfig = parseConfig(
                {
                  ...groupConfig,
                  components: componentConfigs,
                  id: groupId,
                },
                GroupConfigSchemaP1,
                variables,
              );

              setConfig('value', barKey, groupKey, parsedConfig);
            });

            for (const [index, componentConfig] of componentConfigs.entries()) {
              const variables = getElementVariables(componentConfig);
              const componentId = `${barId}-${groupId}-${index}`;

              createComputed(() => {
                const parsedConfig = parseConfig(
                  { ...componentConfig, id: componentId },
                  ComponentConfigSchemaP1,
                  variables,
                );

                setConfig(
                  'value',
                  barKey,
                  groupKey,
                  'components',
                  index,
                  parsedConfig,
                );
              });
            }
          }
        }
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

  function getBarConfigs(userConfig: UserConfig) {
    return Object.entries(userConfig).filter(
      ([key, value]) => key.startsWith('bar/') && !!value,
    ) as [`bar/${string}`, BarConfig][];
  }

  function getGroupConfigs(barConfig: BarConfig) {
    return Object.entries(barConfig).filter(
      ([key, value]) => key.startsWith('group/') && !!value,
    ) as [`group/${string}`, GroupConfig][];
  }

  // TODO: Use generics.
  function parseConfig(
    config: Record<string, unknown>,
    schema: any,
    variables: Record<string, unknown>,
  ) {
    const compiledConfig = Object.entries(config).map(([key, value]) => {
      if (typeof value === 'string') {
        return [key, templateEngine.compile(value, variables)];
      }

      return [key, value];
    });

    return schema.parse(Object.fromEntries(compiledConfig));
  }

  return config;
}
