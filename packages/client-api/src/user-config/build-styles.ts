import { compileString } from 'sass';

import { ElementContext } from '~/context';
import { createLogger } from '~/utils';
import { GlobalConfig } from './types/global-config.model';

const logger = createLogger('build-styles');

/**
 * Compile user-defined SCSS to CSS to be added to the DOM later. Traverse the
 * window config and aggregate all `styles` properties.
 */
export function buildStyles(
  globalConfig: GlobalConfig,
  windowContext: ElementContext,
) {
  const styles = [scopeWith(':root', globalConfig.root_styles)];

  // Queue of element contexts to traverse.
  const queue = [windowContext];

  while (queue.length) {
    const elementContext = queue.shift()!;
    const elementStyles = elementContext.parsedConfig.styles;

    if (elementStyles) {
      styles.push(scopeWith(`#${elementContext.id}`, elementStyles));
    }

    const children = elementContext.children;
    queue.concat(children);
  }

  // Compile SCSS into CSS.
  const { css } = compileString(styles.join(''));
  logger.debug('Compiled SCSS into CSS:', css);

  return css;
}

/**
 * Wrap user-defined styles in a scope.
 */
function scopeWith(selector: string, styles: string | undefined) {
  return styles ? `${selector} { ${styles} }` : '';
}
