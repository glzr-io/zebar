if (window.location.host === '127.0.0.1:3030') {
  document.addEventListener('DOMContentLoaded', () => {
    addFavicon();
    loadCss('/__zebar/normalize.css');
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
  document.head.insertBefore(
    element,
    resources[resources.length - 1] ?? null,
  );
}
