console.log('Service worker loaded', Math.random());

let configPromise = new Promise(resolve => {
  // Store resolver globally so we can call it when config arrives.
  self.configResolver = resolve;
});

self.addEventListener('install', () => {
  // Skip waiting for activation. Only has an effect if there's a newly
  // installed service worker that would otherwise remain in the `waiting`
  // state.
  self.skipWaiting();
});

self.addEventListener('activate', () => {
  // Claim clients to ensure that updates to the underlying service worker
  // take effect immediately. Normally when a service worker is updated,
  // pages won't use it until the next load.
  self.clients.claim();
});

self.addEventListener('fetch', event => {
  // Use the default browser handling for requests where:
  // - The request method is not GET.
  // - The request is a navigation request.
  // - The request is to the same origin as the service worker.
  if (
    event.request.method !== 'GET' ||
    event.request.mode === 'navigate' ||
    new URL(event.request.url).origin === self.location.origin
  ) {
    return;
  }

  event.respondWith(handleFetch(event));
});

self.addEventListener('message', event => {
  switch (event.data.type) {
    case 'CLEAR_CACHE':
      event.waitUntil(
        // TODO: This doesn't work.
        caches.open('v1').then(cache => cache.delete(event.data)),
      );
      break;
    case 'SET_CONFIG':
      // TODO: This won't update the config if another message is received.
      self.configResolver(event.data.config);
      break;
    default:
      console.error(
        'Service worker received unknown message type:',
        event.data,
      );
  }
});

async function handleFetch(event) {
  // Wait for config to be set before processing any requests.
  const config = await configPromise;
  console.log('config', config);

  // First, try to get the resource from the cache.
  const cache = await caches.open('v1');
  const cachedResponse = await cache.match(event.request);

  if (cachedResponse) {
    return cachedResponse;
  }

  try {
    // Otherwise, fetch the resource from the network.
    const networkResponse = await fetch(event.request);

    // Cache the response if its status is in the 200-299 range or if
    // it's opaque. Opaque responses are from requests with 'no-cors',
    // and have a status of 0.
    if (
      networkResponse &&
      (networkResponse.ok || networkResponse.type === 'opaque')
    ) {
      await cache.put(event.request, networkResponse.clone());
    }

    return networkResponse;
  } catch (error) {
    return new Response('Offline or network error occurred.', {
      status: 503,
      statusText: 'Service Unavailable',
      headers: new Headers({ 'Content-Type': 'text/plain' }),
    });
  }
}
