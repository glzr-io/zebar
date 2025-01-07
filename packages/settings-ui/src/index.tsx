/* @refresh reload */
import './index.css';
import { HashRouter, Route } from '@solidjs/router';
import { render } from 'solid-js/web';

import { WidgetPacks } from './configs/WidgetPacks';
import { AppLayout, UserWidgetPacksProvider } from './common';

render(
  () => (
    <UserWidgetPacksProvider>
      <AppLayout>
        <HashRouter>
          <Route path="/" component={WidgetPacks} />
          <Route path="/widget/:path" component={WidgetPacks} />
        </HashRouter>
      </AppLayout>
    </UserWidgetPacksProvider>
  ),
  document.getElementById('root')!,
);
