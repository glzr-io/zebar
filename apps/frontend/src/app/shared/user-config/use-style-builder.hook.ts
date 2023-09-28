import { createResource } from 'solid-js';
import { compileString } from 'sass';

import { useLogger } from '../logging';
import { memoize } from '../utils';
import { useUserConfig } from './use-user-config.hook';
import { getGroupConfigs } from './utils/get-group-configs';
import { getBarConfigs } from './utils/get-bar-configs';

/**
 * Hook for compiling user-provided SCSS to CSS.
 */
export const useStyleBuilder = memoize(() => {
  const logger = useLogger('useStyleBuilder');
  const userConfig = useUserConfig();

  // Traverse the bar config and aggregate all `styles` properties. Compile the
  // result from SCSS -> CSS to be added to the DOM later.
  const [builtCss, { refetch: rebuild }] = createResource(
    () => userConfig.config,
    async userConfig => {
      const bars = getBarConfigs(userConfig);
      const barStyles = bars.map(bar => scopeWith(`#${bar.id}`, bar?.styles));

      const groups = bars.flatMap(bar => getGroupConfigs(bar));
      const groupStyles = groups.map(group =>
        scopeWith(`#${group.id}`, group?.styles),
      );

      const componentStyles = groups
        .flatMap(group => group.components ?? [])
        .map(component => scopeWith(`#${component.id}`, component?.styles));

      const styles = [
        scopeWith(':root', userConfig.general.root_styles),
        ...barStyles,
        ...groupStyles,
        ...componentStyles,
      ].join('');

      // Compile SCSS into CSS.
      const { css } = compileString(styles);
      logger.debug('Compiled SCSS into CSS:', css);

      return css;
    },
  );

  // Wrap user-defined styles in a scope.
  function scopeWith(selector: string, styles: string | undefined) {
    return styles ? `${selector} { ${styles} }` : '';
  }

  return {
    builtCss,
    rebuild,
  };
});
