/* @refresh reload */
import { render } from 'solid-js/web';
import { initAsync } from 'zebar';

import './normalize.scss';
import './index.scss';
import { WindowElement } from './app/window-element.component';
import { createRoot } from 'solid-js';

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error('Root element not found.');
}

// createRoot(() => {
//   initAsync().then(context => {
//     render(() => <WindowElement context={context} />, root!);
//   });
// });

// createRoot(() => {
render(() => <WindowElement />, root!);
// });
