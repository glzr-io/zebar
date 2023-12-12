import { compileString } from 'sass';
import { createEffect, createSignal } from 'solid-js';
import { createStore } from 'solid-js/store';

import { createLogger, memoize, toCssSelector } from '~/utils';

const logger = createLogger('style-builder');

/**
 * Compile user-defined SCSS to CSS to be added to the DOM later. Traverse the
 * window config and aggregate all `styles` properties.
 */
export const useStyleBuilder = memoize(() => {
  const [globalStyles, setGlobalStyles] = createSignal<string | null>(null);

  const [elementStyleMap, setElementStyleMap] = createStore<
    Record<string, string>
  >({});

  // Dynamically create <style> tag and append it to <head>.
  const styleElement = document.createElement('style');
  styleElement.setAttribute('data-zebar', '');
  document.head.appendChild(styleElement);

  createEffect(() => (styleElement.innerHTML = getCompiledCss()));

  // Compile SCSS into CSS.
  function getCompiledCss() {
    const styles: string[] = [];

    if (globalStyles()) {
      styles.push(scopeWith(':root', globalStyles()!));
    }

    for (const [id, elStyles] of Object.entries(elementStyleMap)) {
      styles.push(scopeWith(`#${toCssSelector(id)}`, elStyles));
    }

    const { css } = compileString(styles.join(''));
    logger.debug('Compiled SCSS into CSS:', css);

    return css;
  }

  function setElementStyles(id: string, styles: string) {
    setElementStyleMap({ [id]: styles });
  }

  return {
    setGlobalStyles,
    setElementStyles,
  };
});

/**
 * Wrap user-defined styles in a scope.
 */
function scopeWith(selector: string, styles: string | undefined) {
  return styles ? `${selector} { ${styles} }` : '';
}
