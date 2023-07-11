import { createResource } from 'solid-js';
import { compileString } from 'sass';

import { useLogger } from '../logging';
import { isDefined, memoize, resolved } from '../utils';
import { useUserConfig } from './use-user-config.hook';
import { ComponentGroupConfig } from './types/bar/component-group-config.model';
import { BarConfig } from './types/bar/bar-config.model';

export const useStyleBuilder = memoize(() => {
  const logger = useLogger('useStyleBuilder');
  const userConfig = useUserConfig();

  // Traverse the bar config and aggregate `styles`. Compile this and add
  // it to the DOM somehow.
  const [builtCss] = createResource(
    () => resolved([userConfig.generalConfig(), userConfig.barConfig()]),
    async ([generalConfig, barConfig]) => {
      const groups = getGroups(barConfig);
      const groupStyles = groups.map(group => group?.styles);

      const componentStyles = groups
        .flatMap(group => group.components)
        .map(component => component?.styles);

      // TODO: Merge with default styles.
      // TODO: Add scopes to default styles.
      // TODO: Add scopes to user-defined styles.
      const styles = [
        generalConfig.global_styles,
        barConfig.styles,
        ...groupStyles,
        ...componentStyles,
      ]
        .filter(isDefined)
        .join('');

      // Compile SCSS into CSS.
      const { css } = compileString(styles);
      return css;
    },
  );

  function getGroups(barConfig: BarConfig) {
    return Object.entries(barConfig)
      .filter(([key]) => key.startsWith('group'))
      .map(([_, value]) => value) as ComponentGroupConfig[];
  }

  return {
    builtCss,
  };
});
