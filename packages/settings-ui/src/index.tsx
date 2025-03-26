/* @refresh reload */
import './index.css';
import { HashRouter, Route } from '@solidjs/router';
import { render } from 'solid-js/web';

import {
  ApiClientProvider,
  AppLayout,
  MarketplacePacksProvider,
  UserPacksProvider,
  WidgetPreviewProvider,
} from './common';
import { MarketplacePage, MarketplacePackPage } from './marketplace';
import { WidgetPage, WidgetPacksPage, WidgetPackPage } from './user-packs';

render(
  () => (
    <ApiClientProvider apiBaseUrl={import.meta.env.VITE_API_URL}>
      <UserPacksProvider>
        <MarketplacePacksProvider>
          <WidgetPreviewProvider>
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
          </WidgetPreviewProvider>
        </MarketplacePacksProvider>
      </UserPacksProvider>
    </ApiClientProvider>
  ),
  document.getElementById('root')!,
);
