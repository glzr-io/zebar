import { Resource, createEffect, createRoot, on } from 'solid-js';
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

export interface ConfigStore {
  value: UserConfig | null;
  hasInitialized: boolean;
}

export function createConfigStore(configObj: Resource<unknown>) {
  const templateEngine = useTemplateEngine();

  const [config, setConfig] = createStore<ConfigStore>({
    value: null,
    hasInitialized: false,
  });

  createEffect(() => {
    if (!configObj()) {
      return;
    }

    let dispose: () => void;

    createRoot(dispose => {
      try {
        dispose = dispose;

        setConfig('value', UserConfigP1Schema.parse(configObj()));

        const barConfigs = Object.entries(configObj() as UserConfig).filter(
          ([e]) => e.startsWith('bar/') && e,
        ) as [string, BarConfig][];

        for (const [barKey, barConfig] of barConfigs) {
          const variables = getElementVariables(barConfig);

          function aa() {
            const barConfigEntries = Object.entries(barConfig).map(
              ([key, value]) => {
                if (typeof value === 'string') {
                  return [key, templateEngine.compile(value, variables)];
                } else {
                  return [key, value];
                }
              },
            );
            console.log('barConfigEntries', barConfigEntries, barConfigs);

            const parsedBarConfig = BarConfigSchemaP1.parse(
              Object.fromEntries(barConfigEntries),
            );

            console.log('variables changed (bar)', variables, parsedBarConfig);

            setConfig('value', barKey as `bar/${string}`, parsedBarConfig);
          }

          aa();
          createEffect(on(() => variables, aa, { defer: true }));

          const groupConfigs = Object.entries(barConfig).filter(
            ([e]) => e.startsWith('group/') && e,
          ) as [string, GroupConfig][];

          for (const [groupKey, groupConfig] of groupConfigs) {
            const variables = getElementVariables(groupConfig);

            function bb() {
              const groupConfigEntries = Object.entries(groupConfig).map(
                ([key, value]) => {
                  if (typeof value === 'string') {
                    return [key, templateEngine.compile(value, variables)];
                  } else {
                    return [key, value];
                  }
                },
              );

              const parsedGroupConfig = GroupConfigSchemaP1.parse(
                Object.fromEntries(groupConfigEntries),
              );

              console.log(
                'variables changed (group)',
                variables,
                parsedGroupConfig,
              );

              setConfig(
                'value',
                barKey as `bar/${string}`,
                groupKey as `group/${string}`,
                { ...parsedGroupConfig, components: [] },
              );
            }

            bb();
            createEffect(on(() => variables, bb, { defer: true }));

            for (const [index, componentConfig] of (
              groupConfig.components ?? []
            ).entries()) {
              const variables = getElementVariables(componentConfig);

              function cc() {
                const componentConfigEntries = Object.entries(
                  componentConfig,
                ).map(([key, value]) => {
                  if (typeof value === 'string') {
                    return [key, templateEngine.compile(value, variables)];
                  } else {
                    return [key, value];
                  }
                });

                console.log('componentConfigEntries', componentConfigEntries);

                const parsedComponentConfig = ComponentConfigSchemaP1.parse(
                  Object.fromEntries(componentConfigEntries),
                );

                console.log(
                  'variables changed (component)',
                  variables,
                  parsedComponentConfig,
                );

                setConfig(
                  'value',
                  barKey as `bar/${string}`,
                  groupKey as `group/${string}`,
                  'components',
                  index,
                  parsedComponentConfig,
                );
              }
              cc();
              createEffect(on(() => variables, cc, { defer: true }));
            }
          }
        }

        console.log('config', config);

        setConfig('hasInitialized', true);
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
