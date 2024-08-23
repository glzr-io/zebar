interface Window {
  __TAURI__: any;
  __ZEBAR_INITIAL_STATE: import('./desktop').OpenWindowArgs;
}

declare module '*.css' {
  const src: string;
  export default src;
}
