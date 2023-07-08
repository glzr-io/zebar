import { Accessor } from 'solid-js';
import { render } from 'solid-js/web';

export function insertAndReplace(parent: Element, accessor: Accessor<Element>) {
  parent.innerHTML = '';
  const dispose = render(accessor, parent);
  parent.replaceWith(parent.firstChild!);
  return dispose;
}
