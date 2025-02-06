/* @refresh reload */
import './index.css';
import { HashRouter, Route } from '@solidjs/router';
import { render } from 'solid-js/web';

import { WidgetConfigs } from './configs/WidgetConfigs';

render(
  () => (
    <HashRouter>
      <Route path="/" component={WidgetConfigs} />
      <Route path="/widget/:path" component={WidgetConfigs} />
    </HashRouter>
  ),
  document.getElementById('root')!,
);
