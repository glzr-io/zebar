self.addEventListener('install', event => {
  self.skipWaiting();
});

self.addEventListener('activate', event => {
  self.clients.claim();
});

self.addEventListener('fetch', event => {
  console.log('fetch', event.request);
  // Only cache GET requests.
  if (event.request.method !== 'GET') {
    return;
  }

  event.respondWith(
    (async () => {
      // First try to get the resource from the cache.
      const cache = await caches.open('v1');
      const cachedResponse = await cache.match(event.request);

      if (cachedResponse) {
        return cachedResponse;
      }

      try {
        // Otherwise, fetch the resource from the network.
        const networkResponse = new Request(event.request, {
          headers: {
            ...Object.fromEntries(event.request.headers.entries()),
            'X-Zebar-Token': new URL(location).searchParams.get(
              'widget-token',
            ),
          },
        });

        if (
          networkResponse &&
          networkResponse.status === 200 &&
          networkResponse.type !== 'opaque'
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
