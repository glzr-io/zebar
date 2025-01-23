/* @refresh reload */
import './index.css';
import { HashRouter, Navigate, Route } from '@solidjs/router';
import { render } from 'solid-js/web';

import { AppLayout, UserWidgetPacksProvider } from './common';
import { WidgetPack, WidgetPacks } from './configs';

render(
  () => (
    <UserWidgetPacksProvider>
      <AppLayout>
        <HashRouter>
          <Route
            path="/"
            component={() => <Navigate href="/widget-packs" />}
          />
          <Route path="/widget-packs" component={WidgetPacks} />
          <Route path="/widget-packs/:path" component={WidgetPack} />
        </HashRouter>
      </AppLayout>
    </UserWidgetPacksProvider>
  ),
  document.getElementById('root')!,
);
