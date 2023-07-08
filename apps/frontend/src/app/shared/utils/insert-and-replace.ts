import { Accessor, JSXElement } from 'solid-js';
import { render } from 'solid-js/web';

export function insertAndReplace(
  parent: Element,
  replacement: Accessor<Element | JSXElement>,
) {
  // Delete the existing children from the parent.
  parent.innerHTML = '';

  // Render the new element and replace the parent with it.
  const dispose = render(replacement, parent);
  parent.replaceWith(...Array.from(parent.childNodes));

  return dispose;
}
