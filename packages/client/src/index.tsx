/* @refresh reload */
import { render } from 'solid-js/web';

import './normalize.css';
import './index.css';
import { WindowElement } from './app/window-element.component';

const root = document.getElementById('zebar');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error('Root element not found.');
}

render(() => <WindowElement />, root!);
