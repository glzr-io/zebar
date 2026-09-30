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
  // - The caller explicitly disables caching.
  // - The request is to the same origin as the service worker.
  if (
    event.request.method !== 'GET' ||
    event.request.mode === 'navigate' ||
    event.request.cache === 'no-store' ||
    new URL(event.request.url).origin === self.location.origin
  ) {
    return;
  }

  event.respondWith(handleFetch(event));
});

self.addEventListener('message', event => {
  switch (event.data.type) {
    case 'CLEAR_CACHE':
      event.waitUntil(clearCache());
      break;
    case 'SET_CONFIG':
      // Older widgets still broadcast this; fetches use the caller's policy.
      break;
    default:
      console.error(
        'Service worker received unknown message type:',
        event.data,
      );
  }
});

async function getCacheConfig(clientId) {
  if (!clientId) {
    return null;
  }
  const client = await self.clients.get(clientId);
  if (!client) {
    return null;
  }

  // Request the policy from the originating widget so worker restarts and
  // other widget windows cannot replace it.
  return new Promise(resolve => {
    const channel = new MessageChannel();
    const finish = config => {
      clearTimeout(timeout);
      channel.port1.close();
      channel.port2.close();
      resolve(config);
    };
    const timeout = setTimeout(() => finish(null), 1000);
    channel.port1.onmessage = event => finish(event.data);
    try {
      client.postMessage({ type: 'GET_CACHE_CONFIG' }, [channel.port2]);
    } catch {
      finish(null);
    }
  });
}

async function clearCache() {
  await Promise.all(
    ['responses-v1', 'metadata-v1'].map(cacheName =>
      caches.delete(cacheName),
    ),
  );
}

async function handleFetch(event) {
  const config = await getCacheConfig(event.clientId);
  const duration = config
    ? getCacheDuration(event.request.url, config)
    : 0;
  if (duration === 0) {
    return fetch(event.request);
  }

  const [responseCache, metadataCache] = await Promise.all([
    caches.open('responses-v1'),
    caches.open('metadata-v1'),
  ]);

  // First, try to get the resource and its metadata from the cache.
  const [cachedResponse, cachedMetadata] = await Promise.all([
    responseCache.match(event.request),
    metadataCache.match(event.request).then(res => res?.json()),
  ]);

  // Check if there's a valid cached response.
  if (cachedResponse) {
    const hasExpired =
      !cachedMetadata || Date.now() >= cachedMetadata.timestamp + duration;

    if (!hasExpired) {
      return cachedResponse;
    }

    // If expired, delete it.
    await Promise.all([
      responseCache.delete(event.request),
      metadataCache.delete(event.request),
    ]);
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
      const metadata = {
        timestamp: Date.now(),
      };

      await Promise.all([
        responseCache.put(event.request, networkResponse.clone()),
        metadataCache.put(
          event.request,
          new Response(JSON.stringify(metadata)),
        ),
      ]);
    }

    return networkResponse;
  } catch (error) {
    console.error(error);
    return new Response('Offline or network error occurred.', {
      status: 503,
      statusText: 'Service Unavailable',
      headers: new Headers({ 'Content-Type': 'text/plain' }),
    });
  }
}

/**
 * Gets the cache duration (in milliseconds) for a URL.
 */
function getCacheDuration(url, config) {
  for (const rule of config.rules) {
    if (new RegExp(rule.urlRegex).test(url)) {
      return rule.duration * 1000;
    }
  }

  return config.defaultDuration * 1000;
}
