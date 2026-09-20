// Only the first logged-in user's GlazeWM gets port 6123, since a port is
// machine-wide. The rest listen on a port picked at runtime. Widgets load
// `glazewm-js` straight from a CDN with 6123 baked in, so those connections
// are pointed at the running WM here instead of every widget being patched.
// On a machine with one user this whole block is skipped.
if (window.__ZEBAR_PORTS.glazewmIpc !== 6123) {
  const NativeWebSocket = window.WebSocket;

  window.WebSocket = class extends NativeWebSocket {
    constructor(url, protocols) {
      super(
        String(url).replace(
          /^(wss?:\/\/(?:localhost|127\.0\.0\.1)):6123(?=\/|$)/,
          `$1:${window.__ZEBAR_PORTS.glazewmIpc}`,
        ),
        protocols,
      );
    }
  };
}

// Clear console every 15 minutes.
setInterval(
  () => {
    console.clear();
    console.info(
      '%c[Zebar]%c Console is cleared every 15 minutes to prevent memory buildup from logged data.',
      'color: #4ade80',
      'color: inherit',
    );
  },
  1000 * 60 * 15,
);

if (
  window.location.host === `127.0.0.1:${window.__ZEBAR_PORTS.assetServer}`
) {
  if ('serviceWorker' in navigator) {
    navigator.serviceWorker
      .register('/__zebar/sw.js', { scope: '/' })
      .then(sw => {
        console.info(
          '%c[Zebar]%c Service Worker registered.',
          'color: #4ade80',
          'color: inherit',
        );

        const message = {
          type: 'SET_CONFIG',
          config: window.__ZEBAR_STATE.config.caching,
        };

        sw.active?.postMessage(message);
        sw.installing?.postMessage(message);
        sw.waiting?.postMessage(message);
      })
      .catch(err =>
        console.error(
          '%c[Zebar]%c Service Worker failed to register:',
          'color: #4ade80',
          'color: inherit',
          err,
        ),
      );
  }

  document.addEventListener('DOMContentLoaded', () => {
    addFavicon();
    loadCss('/__zebar/normalize.css');
  });
}

/**
 * Adds a CSS file with the given path to the head element.
 */
function loadCss(path) {
  const link = document.createElement('link');
  link.setAttribute('data-zebar', '');
  link.rel = 'stylesheet';
  link.type = 'text/css';
  link.href = path;
  insertIntoHead(link);
}

/**
 * Adds a favicon to the head element if one is not already present.
 */
function addFavicon() {
  if (!document.querySelector('link[rel="icon"]')) {
    const link = document.createElement('link');
    link.setAttribute('data-zebar', '');
    link.rel = 'icon';
    link.href = 'data:;';
    insertIntoHead(link);
  }
}

/**
 * Inserts the element before any other resource tags in the head element.
 * Ensures that user-defined stylesheets or favicons are prioritized over
 * Zebar's defaults.
 */
function insertIntoHead(element) {
  const resources = document.head.querySelectorAll('link, script, style');
  const target = resources[0]?.previousElementSibling;

  if (target) {
    target.after(element);
  } else {
    document.head.appendChild(element);
  }
}
