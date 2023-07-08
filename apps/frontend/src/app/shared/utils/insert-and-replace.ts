import { Accessor } from 'solid-js';
import { insert } from 'solid-js/web';

export function insertAndReplace(parent: Element, accessor: Accessor<Element>) {
  parent.innerHTML = '';
  insert(parent, accessor);
  parent.replaceWith(parent.firstChild!);
}
