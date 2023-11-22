interface Window {
  __TAURI__: any;
  __ZEBAR_FNS: Record<string, Function>;
}

declare module '*.html' {
  const src: string;
  export default src;
}
