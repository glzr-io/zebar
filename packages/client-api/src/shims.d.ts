interface Window {
  __TAURI__: any;
  __ZEBAR_FNS: Record<string, Function>;
  __ZEBAR_INIT_STATE: import('./desktop').InitialState;
}

declare module '*.html' {
  const src: string;
  export default src;
}
