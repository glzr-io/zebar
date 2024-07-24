import { createLogger, toCssSelector } from '~/utils';

const logger = createLogger('style-builder');

let globalStyles: string | null = null;
let elementStyles: Record<string, string> = {};
let styleElement: HTMLStyleElement | null = null;

/**
 * Abstraction over building CSS from user-defined styles.
 */
export function getStyleBuilder() {
  function buildGlobalStyles(styles: string) {
    logger.debug(`Updating global CSS:`, styles);

    globalStyles = styles;
    buildStyles();
  }

  function buildElementStyles(id: string, styles: string) {
    // Wrap user-defined styles in a scope.
    const scopedStyles = `#${toCssSelector(id)} {\n${styles}}`;
    logger.debug(`Updating element '${id}' CSS:\n`, scopedStyles);

    elementStyles[id] = scopedStyles;
    buildStyles();
  }

  /**
   * Compile user-defined CSS and add it to the DOM.
   */
  function buildStyles() {
    if (!styleElement) {
      styleElement = document.createElement('style');
      styleElement.setAttribute('data-zebar', '');
      document.head.appendChild(styleElement);
    }

    const styles = [globalStyles ?? '', ...Object.values(elementStyles)];
    styleElement.innerHTML = styles.join('\n');
  }

  return {
    buildGlobalStyles,
    buildElementStyles,
  };
}
