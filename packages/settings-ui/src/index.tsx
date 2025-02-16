/* @refresh reload */
import './index.css';
import { HashRouter, Route } from '@solidjs/router';
import { render } from 'solid-js/web';

import {
  AppLayout,
  MarketplacePacksProvider,
  UserPacksProvider,
} from './common';
import { Marketplace, MarketplacePack } from './marketplace';
import { WidgetPack, WidgetPacks } from './user-packs';

render(
  () => (
    <UserPacksProvider>
      <MarketplacePacksProvider>
        <HashRouter root={AppLayout}>
          <Route path="/" component={WidgetPacks} />
          <Route path="/packs/:packId" component={WidgetPack} />
          <Route path="/packs/:packId/:widgetId" component={WidgetPack} />
          <Route path="/marketplace" component={Marketplace} />
          <Route
            path="/marketplace/packs/:id"
            component={MarketplacePack}
          />
        </HashRouter>
      </MarketplacePacksProvider>
    </UserPacksProvider>
  ),
  document.getElementById('root')!,
);
