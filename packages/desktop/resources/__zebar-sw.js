const encodeUserAgent = userAgent => {
  if (userAgent.includes('zebar')) {
    return userAgent;
  }
  return btoa(userAgent)
    .replace(/=/g, '')
    .replace(/\+/g, '-')
    .replace(/\//g, '_');
};

self.addEventListener('install', event => {
  self.skipWaiting();
});

self.addEventListener('activate', event => {
  self.clients.claim();
});

self.addEventListener('fetch', event => {
  if (event.request.method !== 'GET') return;

  event.respondWith(
    (async () => {
      const userAgent =
        event.request.headers.get('User-Agent') || '__zebar-service-worker';

      const encodedUserAgent = encodeUserAgent(userAgent);
      const CACHE_NAME = encodedUserAgent;

      try {
        const cache = await caches.open(CACHE_NAME);

        const cachedResponse = await cache.match(event.request);
        if (cachedResponse) {
          return cachedResponse;
        }

        const networkResponse = await fetch(event.request);

        if (
          networkResponse &&
          networkResponse.status === 200 &&
          networkResponse.type !== 'opaque'
        ) {
          const responseToCache = networkResponse.clone();

          await cache.put(event.request, responseToCache);
        }

        return networkResponse;
      } catch (error) {
        return new Response('Offline or network error occurred.', {
          status: 503,
          statusText: 'Service Unavailable',
          headers: new Headers({ 'Content-Type': 'text/plain' }),
        });
      }
    })(),
  );
});

self.addEventListener('message', event => {
  const { type, key } = event.data;

  if (type === 'CLEAR_CACHE' && typeof key === 'string') {
    clearCacheByKey(key);
  } else {
    console.warn(
      'Received unknown message type or invalid key:',
      event.data,
    );
  }
});

const clearCacheByKey = async key => {
  try {
    const success = await caches.delete(key);
    if (success) {
      console.log(`Cache '${key}' deleted successfully.`);
    } else {
      console.log(`Cache '${key}' not found or could not be deleted.`);
    }
  } catch (error) {
    console.error(`Error deleting cache '${key}':`, error);
  }
};
