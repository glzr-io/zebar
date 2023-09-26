import { createResource } from 'solid-js';
import { compileString } from 'sass';

import { useLogger } from '../logging';
import { memoize, resolved } from '../utils';
import { useUserConfig } from './use-user-config.hook';
import { getGroupConfigs } from './utils/get-group-configs';

/**
 * Hook for compiling user-provided SCSS to CSS.
 */
export const useStyleBuilder = memoize(() => {
  const logger = useLogger('useStyleBuilder');
  const userConfig = useUserConfig();

  // Traverse the bar config and aggregate all `styles` properties. Compile the
  // result from SCSS -> CSS to be added to the DOM later.
  const [builtCss, { refetch: rebuild }] = createResource(
    () => resolved([userConfig.generalConfig(), userConfig.barConfig()]),
    async ([generalConfig, barConfig]) => {
      const groups = getGroupConfigs(barConfig);

      const groupStyles = groups.map(group =>
        scopeWith(`#${group.id}`, group?.styles),
      );

      const componentStyles = groups
        .flatMap(group => group.components ?? [])
        .map(component => scopeWith(`#${component.id}`, component?.styles));

      const styles = [
        scopeWith(':root', generalConfig.root_styles),
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

  // Wrap user-defined styles in a scope.
  function scopeWith(selector: string, styles: string | undefined) {
    return styles ? `${selector} { ${styles} }` : '';
  }

  return {
    builtCss,
    rebuild,
  };
});
