import { createRoot } from 'solid-js';

/**
 * Replace an element with one or more elements. Creates a reactive scope around
 * the mounted element(s).
 */
export function mount(
  mountEl: Element | null,
  replacement: Element | Element[],
): () => void {
  if (!(mountEl instanceof HTMLElement)) {
    throw new Error(`Unable to mount element '${mountEl}'.`);
  }

  let disposer: () => void;

  createRoot(dispose => {
    disposer = dispose;

    const replacementEls = Array.isArray(replacement)
      ? replacement
      : [replacement];

    mountEl.replaceWith(...replacementEls);
  });

  return disposer!;
}
