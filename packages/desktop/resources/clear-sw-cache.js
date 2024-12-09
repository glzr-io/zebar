if ('serviceWorker' in navigator) {
  // Get the active service worker registration.
  navigator.serviceWorker.ready.then(sw => {
    sw.active?.postMessage({ type: 'CLEAR_CACHE' });
  });
}
