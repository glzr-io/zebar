interface Window {
  __TAURI__: any;
  __ZEBAR_INITIAL_STATE: import('./desktop').WindowState;
}

declare module '*.css' {
  const src: string;
  export default src;
}
