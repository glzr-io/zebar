import { compileString } from 'sass';

import { createLogger } from '~/utils';
import { GlobalConfig } from './types/global-config.model';
import { WindowConfig } from './types/window/window-config.model';
import { BaseElementConfig } from './types/window/base-element-config.model';

const logger = createLogger('build-styles');

/**
 * Compile user-defined SCSS to CSS to be added to the DOM later. Traverse the
 * window config and aggregate all `styles` properties.
 */
export function buildStyles(
  globalConfig: GlobalConfig,
  windowConfig: WindowConfig,
) {
  const styles = [scopeWith(':root', globalConfig.root_styles)];

  // Queue of elements to traverse.
  const queue: BaseElementConfig[] = [windowConfig];

  while (queue.length) {
    const elementConfig = queue.shift()!;

    if (elementConfig.styles) {
      styles.push(scopeWith(`#${elementConfig.id}`, elementConfig.styles));
    }

    // TODO: How to get children? Filter by `group/` and `template/` keys?
    // const children = elementConfig.children;
    // queue.concat(children);
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
