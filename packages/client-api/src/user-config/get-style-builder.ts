import { compileString } from 'sass';
import {
  type Owner,
  createEffect,
  createSignal,
  runWithOwner,
} from 'solid-js';
import { createStore } from 'solid-js/store';

import { createLogger, toCssSelector } from '~/utils';

const logger = createLogger('style-builder');

const [globalStyles, setGlobalStyles] = createSignal<string | null>(null);
const [elementStyles, setElementStyles] = createStore<
  Record<string, string>
>({});

/**
 * Abstraction over building CSS from user-defined styles.
 */
export function getStyleBuilder(owner: Owner) {
  const hasInitialized = document.querySelector('style[data-zebar]');

  // Listen to changes to changes in global + element styles, and dynamically
  // append built <style> tag to document head.
  if (!hasInitialized) {
    const styleElement = document.createElement('style');
    styleElement.setAttribute('data-zebar', '');
    document.head.appendChild(styleElement);

    runWithOwner(owner, () => {
      createEffect(() => (styleElement.innerHTML = getCompiledCss()));
    });
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
  const styles: string[] = [];

  if (globalStyles()) {
    styles.push(scopeWith(':root', globalStyles()!));
  }

  for (const [id, elStyles] of Object.entries(elementStyles)) {
    styles.push(scopeWith(`#${toCssSelector(id)}`, elStyles));
  }

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
