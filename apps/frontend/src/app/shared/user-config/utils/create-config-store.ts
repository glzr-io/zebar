import { Resource, createComputed, createEffect, createRoot } from 'solid-js';
import { createStore } from 'solid-js/store';
import { z } from 'zod';

import { UserConfig } from '../types/user-config.model';
import { ProvidersConfigSchema } from '../types/bar/providers-config.model';
import { useProvider } from '~/shared/providers';
import { BaseElementConfig } from '../types/bar/base-element-config.model';
import { formatConfigError } from './format-config-error';
import { useTemplateEngine } from '../use-template-engine.hook';
import { BarConfigSchemaP1 } from '../types/bar/bar-config.model';
import { GroupConfigSchemaP1 } from '../types/bar/group-config.model';
import { ComponentConfigSchemaP1 } from '../types/bar/component-config.model';
import { useConfigVariables } from '../use-config-variables.hook';
import { getBarConfigs } from './get-bar-configs';
import { getGroupConfigs } from './get-group-configs';
import { GeneralConfigSchema } from '../types/general-config.model';

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
        updateConfig(configObj() as UserConfig);
      } catch (e) {
        dispose();
        throw formatConfigError(e);
      }
    });

    return () => dispose();
  });

  // Traverse down user config and update config with parsed + validated
  // properties.
  function updateConfig(configObj: UserConfig) {
    const rootVariables = {
      env: configVariables()!,
    };

    // Update general config.
    createComputed(() => {
      const parsedConfig = parseConfig(
        configObj.general,
        GeneralConfigSchema.strip(),
        rootVariables,
      );

      setConfig('value', { general: parsedConfig });
    });

    // Update bar configs.
    for (const [barKey, barConfig] of getBarConfigs(configObj)) {
      const variables = getElementVariables(barConfig);
      const barId = `bar-${barKey.split('/')[1]}`;

      createComputed(() => {
        const parsedConfig = parseConfig(
          { ...barConfig, id: barId },
          BarConfigSchemaP1.strip(),
          variables,
        );

        setConfig('value', barKey, parsedConfig);
      });

      for (const [groupKey, groupConfig] of getGroupConfigs(barConfig)) {
        const variables = getElementVariables(groupConfig);
        const groupId = `${barId}-${groupKey.split('/')[1]}`;

        createComputed(() => {
          const parsedConfig = parseConfig(
            { ...groupConfig, id: groupId },
            GroupConfigSchemaP1.strip(),
            variables,
          );

          setConfig('value', barKey, groupKey, prev => ({
            ...parsedConfig,
            components: prev?.components ?? [],
          }));
        });

        const componentConfigs = (groupConfig.components ?? []).entries();

        for (const [index, componentConfig] of componentConfigs) {
          const variables = getElementVariables(componentConfig);
          const componentId = `${groupId}-${index}`;

          createComputed(() => {
            const parsedConfig = parseConfig(
              { ...componentConfig, id: componentId },
              ComponentConfigSchemaP1.strip(),
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
  }

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

  function parseConfig<
    T extends Record<string, unknown>,
    U extends z.AnyZodObject,
  >(config: T, schema: U, variables: Record<string, unknown>): z.infer<U> {
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
