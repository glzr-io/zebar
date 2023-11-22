import { InitialState } from './desktop';

interface Window {
  __TAURI__: any;
  __ZEBAR_FNS: Record<string, Function>;
  __ZEBAR_INIT_STATE: InitialState;
}

declare module '*.html' {
  const src: string;
  export default src;
}
