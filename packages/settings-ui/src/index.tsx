/* @refresh reload */
import './index.css';
import { HashRouter, Navigate, Route } from '@solidjs/router';
import { render } from 'solid-js/web';

import {
  AppLayout,
  CommunityPacksProvider,
  UserPacksProvider,
} from './common';
import { CommunityPacks } from './community';
import { WidgetPack, WidgetPacks } from './user-packs';

render(
  () => (
    <UserPacksProvider>
      <CommunityPacksProvider>
        <HashRouter root={AppLayout}>
          <Route
            path="/"
            component={() => <Navigate href="/user-packs" />}
          />
          <Route path="/user-packs" component={WidgetPacks} />
          <Route path="/user-packs/:path" component={WidgetPack} />
          <Route path="/community" component={CommunityPacks} />
        </HashRouter>
      </CommunityPacksProvider>
    </UserPacksProvider>
  ),
  document.getElementById('root')!,
);
