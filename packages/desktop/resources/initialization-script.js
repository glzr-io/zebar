if (window.location.host === '127.0.0.1:3030') {
  // Load normalized CSS on DOM ready.
  document.addEventListener('DOMContentLoaded', () => {
    const link = document.createElement('link');
    link.setAttribute('data-zebar', '');
    link.rel = 'stylesheet';
    link.type = 'text/css';
    link.href = '/__zebar/normalize.css';
    document.head.appendChild(link);
  });

  if ('serviceWorker' in navigator) {
    navigator.serviceWorker
      .register('/__zebar/sw.js')
      .then(() => console.log('[Zebar] Service Worker registered.'))
      .catch(err =>
        console.error('[Zebar] Service Worker failed to register:', err),
      );
  }
}
