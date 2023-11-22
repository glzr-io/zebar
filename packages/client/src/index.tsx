/* @refresh reload */
import { Suspense, render } from 'solid-js/web';

import './normalize.scss';
import './index.scss';
import { WindowElement } from './app/window-element.component';
import {
  createComputed,
  createEffect,
  createRenderEffect,
  createResource,
} from 'solid-js';

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error('Root element not found.');
}

render(() => <WindowElement />, root!);
