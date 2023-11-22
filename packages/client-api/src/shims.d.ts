interface Window {
  __TAURI__: any;
  __ZEBAR_FNS: Record<string, Function>;
  // TODO
  __ZEBAR_INIT_STATE: any;
}

declare module '*.html' {
  const src: string;
  export default src;
}
