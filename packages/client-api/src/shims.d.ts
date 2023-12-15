interface Window {
  __TAURI__: any;
  __ZEBAR_OPEN_ARGS: import('./desktop').OpenWindowArgs;
}

declare module '*.html' {
  const src: string;
  export default src;
}
