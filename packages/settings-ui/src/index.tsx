/* @refresh reload */
import './index.css';
import { HashRouter, Route } from '@solidjs/router';
import { render } from 'solid-js/web';

import {
  AppLayout,
  MarketplacePacksProvider,
  UserPacksProvider,
} from './common';
import { MarketplacePage, MarketplacePackPage } from './marketplace';
import { WidgetPage, WidgetPacksPage, WidgetPackPage } from './user-packs';

render(
  () => (
    <UserPacksProvider>
      <MarketplacePacksProvider>
        <HashRouter root={AppLayout}>
          <Route path="/" component={WidgetPacksPage} />
          <Route path="/packs/:packId" component={WidgetPackPage} />
          <Route
            path="/packs/:packId/:widgetName"
            component={WidgetPage}
          />
          <Route path="/marketplace" component={MarketplacePage} />
          <Route
            path="/marketplace/packs/:id"
            component={MarketplacePackPage}
          />
        </HashRouter>
      </MarketplacePacksProvider>
    </UserPacksProvider>
  ),
  document.getElementById('root')!,
);
