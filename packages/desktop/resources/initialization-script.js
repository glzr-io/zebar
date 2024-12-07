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
 * Adds a CSS file with the given path to the head.
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
 * Adds a favicon to the head if one is not already present.
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
 * Inserts the element after the last meta tag. Appends to the end of the
 * head if no meta tags are present.
 */
function insertIntoHead(element) {
  const lastMeta = document.head.querySelector('meta:last-of-type');
  document.head.insertBefore(element, lastMeta?.nextSibling ?? null);
}
