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

/**
 * Config-related state.
 *
 * The cache config is asynchronously resolved by posting a message from
 * the initialization script.
 */
const deferredConfig = {
  value: null,
  resolve: null,
  promise: new Promise(resolve =>
    setTimeout(() => (deferredConfig.resolve = resolve)),
  ),
};

self.addEventListener('message', event => {
  switch (event.data.type) {
    case 'CLEAR_CACHE':
      event.waitUntil(clearCache());
      break;
    case 'SET_CONFIG':
      deferredConfig.value = event.data.config;
      deferredConfig.resolve();
      break;
    default:
      console.error(
        'Service worker received unknown message type:',
        event.data,
      );
  }
});

async function clearCache() {
  await Promise.all(
    ['responses-v1', 'metadata-v1'].map(cacheName =>
      caches.delete(cacheName),
    ),
  );
}

async function handleFetch(event) {
  // Wait for config to be set before processing any requests.
  const config = await deferredConfig.promise.then(
    () => deferredConfig.value,
  );

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
      !cachedMetadata ||
      Date.now() > cachedMetadata.timestamp + cachedMetadata.duration;

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
        duration: getCacheDuration(event.request.url, config),
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
