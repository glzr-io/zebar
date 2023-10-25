import {
  Resource,
  createComputed,
  createEffect,
  createMemo,
  createRoot,
} from 'solid-js';
import { createStore } from 'solid-js/store';
import { z } from 'zod';

import { UserConfig } from '../types/user-config.model';
import { ProvidersConfigSchema } from '../types/bar/providers-config.model';
import { createProvider } from '~/context/providers';
import { BaseElementConfig } from '../types/bar/base-element-config.model';
import { formatConfigError } from './format-config-error';
import { BarConfigSchemaP1 } from '../types/bar/bar-config.model';
import { GroupConfigSchemaP1 } from '../types/bar/group-config.model';
import { ComponentConfigSchemaP1 } from '../types/bar/component-config.model';
import { getConfigVariables } from '../get-config-variables';
import { getBarConfigEntries } from './get-bar-configs';
import { getGroupConfigEntries } from './get-group-configs';
import { TemplateError, useTemplateEngine } from '~/template-engine';
import { TemplatePropertyError } from './template-property-error';
import { GlobalConfigSchema } from '../types/global-config.model';
import { createLogger } from '~/utils';

export interface ConfigStore {
  value: UserConfig | null;
}

export function createConfigStore(configObj: Resource<unknown>) {
  const logger = createLogger('createConfigStore');
  const templateEngine = useTemplateEngine();
  const configVariables = getConfigVariables();

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
    // Update global config.
    setConfig('value', {
      global: parseConfig(configObj.global, GlobalConfigSchema.strip(), {}),
    });

    const rootVariables = createMemo(() => ({
      env: configVariables()!,
    }));

    // Update bar configs.
    for (const [barKey, barConfig] of getBarConfigEntries(configObj)) {
      const barId = `bar-${barKey.split('/')[1]}`;
      const barVariables = createMemo(() => ({
        ...rootVariables(),
        ...getElementVariables(barConfig),
      }));

      createComputed(() => {
        const parsedConfig = parseConfig(
          { ...barConfig, id: barId },
          BarConfigSchemaP1.strip(),
          barVariables(),
        );

        setConfig('value', barKey, parsedConfig);
      });

      // Update group configs.
      for (const [groupKey, groupConfig] of getGroupConfigEntries(barConfig)) {
        const groupId = `${barId}-${groupKey.split('/')[1]}`;
        const groupVariables = createMemo(() => ({
          ...rootVariables(),
          ...getElementVariables(barConfig),
          ...getElementVariables(groupConfig),
        }));

        createComputed(() => {
          const parsedConfig = parseConfig(
            { ...groupConfig, id: groupId },
            GroupConfigSchemaP1.strip(),
            groupVariables(),
          );

          setConfig('value', barKey, groupKey, prev => ({
            ...parsedConfig,
            components: prev?.components ?? [],
          }));
        });

        const componentConfigs = (groupConfig.components ?? []).entries();

        // Update component configs.
        for (const [index, componentConfig] of componentConfigs) {
          const componentId = `${groupId}-${index}`;
          const componentVariables = createMemo(() => ({
            ...rootVariables(),
            ...getElementVariables(barConfig),
            ...getElementVariables(groupConfig),
            ...getElementVariables(componentConfig),
          }));

          createComputed(() => {
            const parsedConfig = parseConfig(
              { ...componentConfig, id: componentId },
              ComponentConfigSchemaP1.strip(),
              componentVariables(),
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
        [config.type]: createProvider(config).variables,
      }),
      {},
    );
  }

  function parseConfig<
    T extends Record<string, unknown>,
    U extends z.AnyZodObject,
  >(config: T, schema: U, variables: Record<string, unknown>): z.infer<U> {
    const newConfigEntries = Object.entries(config).map(([key, value]) => {
      if (typeof value === 'string') {
        try {
          const rendered = templateEngine.render(value, variables);
          return [key, rendered];
        } catch (err) {
          // Re-throw error as `TemplatePropertyError`.
          throw err instanceof TemplateError
            ? new TemplatePropertyError(
                err.message,
                key,
                value,
                err.templateIndex,
              )
            : err;
        }
      }

      return [key, value];
    });

    const newConfig = Object.fromEntries(newConfigEntries);
    logger.debug('Config updated:', newConfig);

    return schema.parse(newConfig);
  }

  return config;
}
