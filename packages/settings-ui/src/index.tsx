/* @refresh reload */
import './index.css';
import { HashRouter, Navigate, Route } from '@solidjs/router';
import { render } from 'solid-js/web';

import { AppLayout, UserPacksProvider } from './common';
import { WidgetPack, WidgetPacks } from './user-packs';

render(
  () => (
    <UserPacksProvider>
      <AppLayout>
        <HashRouter>
          <Route
            path="/"
            component={() => <Navigate href="/user-packs" />}
          />
          <Route path="/user-packs" component={WidgetPacks} />
          <Route path="/user-packs/:path" component={WidgetPack} />
        </HashRouter>
      </AppLayout>
    </UserPacksProvider>
  ),
  document.getElementById('root')!,
);
