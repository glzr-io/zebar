if (window.location.host === '127.0.0.1:3030') {
  if ('serviceWorker' in navigator) {
    navigator.serviceWorker
      .register('/__zebar/sw.js')
      .then(() => console.log('[Zebar] Service Worker registered.'))
      .catch(err =>
        console.error('[Zebar] Service Worker failed to register:', err),
      );
  }
}
