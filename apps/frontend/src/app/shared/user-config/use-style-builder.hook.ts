import { createResource } from 'solid-js';
import { compileString } from 'sass';

import { useLogger } from '../logging';
import { memoize, resolved } from '../utils';
import { useUserConfig } from './use-user-config.hook';
import { GroupConfig } from './types/bar/group-config.model';
import { BarConfig } from './types/bar/bar-config.model';

export const useStyleBuilder = memoize(() => {
  const logger = useLogger(useStyleBuilder.name);
  const userConfig = useUserConfig();

  // Traverse the bar config and aggregate `styles`. Compile this and add
  // it to the DOM somehow.
  const [builtCss] = createResource(
    () => resolved([userConfig.generalConfig(), userConfig.barConfig()]),
    async ([generalConfig, barConfig]) => {
      const groups = getGroupConfigs(barConfig);
      const groupStyles = groups.map(group =>
        scopeWith(`#${group.id}`, group?.styles),
      );

      const defaultBarScss = await importScss('~/bar/bar.scss');
      const defaultBarStyles = defaultBarScss.replace(
        ':host',
        `:where(.${barConfig.class_name})`,
      );

      const componentStyles = groups
        .flatMap(group => group.components ?? [])
        .map(component => scopeWith(`#${component.id}`, component?.styles));

      // TODO: Merge with default styles.
      // TODO: Add scopes to default styles.
      const styles = [
        defaultBarStyles,
        scopeWith(':root', generalConfig.global_styles),
        scopeWith(`#${barConfig.id}`, barConfig.styles),
        ...groupStyles,
        ...componentStyles,
      ].join('');

      // Compile SCSS into CSS.
      const { css } = compileString(styles);
      logger.debug('Compiled SCSS into CSS:', css);

      return css;
    },
  );

  function getGroupConfigs(barConfig: BarConfig) {
    return Object.entries(barConfig)
      .filter(([key, value]) => key.startsWith('group') && !!value)
      .map(([_, value]) => value) as GroupConfig[];
  }

  // Wrap user-defined styles in a scope.
  function scopeWith(selector: string, styles: string | undefined) {
    return styles ? `${selector} { ${styles} }` : '';
  }

  async function importScss(path: string) {
    path = path.replace('~/', '../../');
    const { default: scss } = await import(path);
    return scss as string;
  }

  return {
    builtCss,
  };
});
