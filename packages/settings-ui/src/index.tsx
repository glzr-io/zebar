/* @refresh reload */
import './index.css';
import { Route, Router } from '@solidjs/router';
import { render } from 'solid-js/web';
import { WidgetSettings } from './settings/WidgetSettings';

render(
  () => (
    <Router>
      <Route path="/" component={WidgetSettings} />
    </Router>
  ),
  document.getElementById('root')!,
);
