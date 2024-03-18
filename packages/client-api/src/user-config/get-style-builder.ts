import { compileString } from 'sass';

import { createLogger, toCssSelector } from '~/utils';

const logger = createLogger('style-builder');

let globalStyles: string | null = null;
let elementStyles: Record<string, string> = {};
let styleElement: HTMLStyleElement | null = null;

/**
 * Abstraction over building CSS from user-defined styles.
 */
export function getStyleBuilder() {
  function setGlobalStyles(styles: string) {
    globalStyles = styles;
    buildStyles();
  }

  function setElementStyles(id: string, styles: string) {
    elementStyles[id] = styles;
    buildStyles();
  }

  function buildStyles() {
    if (!styleElement) {
      styleElement = document.createElement('style');
      styleElement.setAttribute('data-zebar', '');
      document.head.appendChild(styleElement);
    }

    styleElement.innerHTML = getCompiledCss();
  }

  return {
    setGlobalStyles,
    setElementStyles,
  };
}

/**
 * Compile user-defined SCSS to CSS to be added to the DOM.
 */
function getCompiledCss(): string {
  try {
    const styles: string[] = [];

    if (globalStyles) {
      styles.push(globalStyles);
    }

    for (const [id, elStyles] of Object.entries(elementStyles)) {
      styles.push(scopeWith(`#${toCssSelector(id)}`, elStyles));
    }

    const { css } = compileString(styles.join('\n'));
    logger.debug('Compiled SCSS into CSS:', css);

    return css;
  } catch (err) {
    // Re-throw error with formatted message.
    throw new Error(`Failed to build CSS: ${(err as Error).message}`);
  }
}

/**
 * Wrap user-defined styles in a scope.
 */
function scopeWith(selector: string, styles: string | undefined) {
  return styles ? `${selector} { ${styles} }` : '';
}
