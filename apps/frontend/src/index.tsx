/* @refresh reload */
import './normalize.scss';
import './index.scss';
import { App } from './app/app.component';
import { mount } from '~/shared/utils';

mount(document.getElementById('root'), App());
