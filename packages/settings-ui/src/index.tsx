/* @refresh reload */
import './index.css';
import { Route, Router } from '@solidjs/router';
import { render } from 'solid-js/web';

import { WidgetConfigs } from './configs/WidgetConfigs';

render(
  () => (
    <Router>
      <Route path="/" component={WidgetConfigs} />
    </Router>
  ),
  document.getElementById('root')!,
);
