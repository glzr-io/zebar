const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const { join } = require('node:path');
const { test } = require('node:test');
const vm = require('node:vm');

function isIntercepted(request) {
  const listeners = new Map();
  const context = {
    URL,
    setTimeout,
    self: {
      location: { origin: 'http://127.0.0.1:6124' },
      clients: { get: () => new Promise(() => {}) },
      addEventListener: (type, listener) => listeners.set(type, listener),
    },
  };
  vm.runInNewContext(
    readFileSync(join(__dirname, '../resources/sw.js'), 'utf8'),
    context,
  );
  let intercepted = false;
  listeners.get('fetch')({
    clientId: 'widget',
    request,
    respondWith: () => {
      intercepted = true;
    },
  });
  return intercepted;
}

test('no-store cross-origin requests bypass the service worker cache', () => {
  assert.equal(
    isIntercepted(
      new Request('http://127.0.0.1:8085/data.json', {
        cache: 'no-store',
      }),
    ),
    false,
  );
});

test('ordinary cross-origin GET requests retain configured caching', () => {
  assert.equal(
    isIntercepted(new Request('https://example.com/data.json')),
    true,
  );
});

test('same-origin and non-GET requests still use browser handling', () => {
  assert.equal(
    isIntercepted(new Request('http://127.0.0.1:6124/widget.js')),
    false,
  );
  assert.equal(
    isIntercepted(
      new Request('https://example.com/data.json', { method: 'POST' }),
    ),
    false,
  );
});
