const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const { join } = require('node:path');
const { test } = require('node:test');
const vm = require('node:vm');

function createWorker({ clients = new Map(), stores = new Map() } = {}) {
  let timestamp = 10000;
  let requests = 0;
  const context = vm.createContext({
    URL,
    Response,
    Headers,
    MessageChannel,
    setTimeout,
    clearTimeout,
    console,
    Date: { now: () => timestamp },
    self: {
      addEventListener: () => {},
      clients: { get: async id => clients.get(id) },
    },
    caches: {
      async open(name) {
        if (!stores.has(name)) stores.set(name, new Map());
        const store = stores.get(name);
        return {
          match: async request => store.get(request.url)?.clone(),
          put: async (request, response) =>
            store.set(request.url, response.clone()),
          delete: async request => store.delete(request.url),
        };
      },
    },
    fetch: async () => Response.json({ count: ++requests }),
  });
  vm.runInContext(
    readFileSync(join(__dirname, '../resources/sw.js'), 'utf8'),
    context,
  );
  const handleFetch = vm.runInContext('handleFetch', context);
  return {
    async read(clientId, path = 'data') {
      return (
        await handleFetch({
          clientId,
          request: new Request(`https://example.com/${path}`),
        })
      ).json();
    },
    advance: milliseconds => {
      timestamp += milliseconds;
    },
    requests: () => requests,
    stores,
  };
}

function widget(defaultDuration, rules = []) {
  return {
    postMessage(message, [port]) {
      assert.equal(message.type, 'GET_CACHE_CONFIG');
      port.postMessage({ defaultDuration, rules });
      port.close();
    },
  };
}

test('widgets with caching disabled bypass shared entries without replacing them', async () => {
  const worker = createWorker({
    clients: new Map([
      ['cached', widget(3600)],
      ['fresh', widget(0)],
    ]),
  });
  assert.deepEqual(await worker.read('cached'), { count: 1 });
  assert.deepEqual(await worker.read('fresh'), { count: 2 });
  assert.deepEqual(await worker.read('fresh'), { count: 3 });
  assert.deepEqual(await worker.read('cached'), { count: 1 });
  assert.deepEqual(await worker.read('fresh', 'new'), { count: 4 });
  assert.deepEqual(await worker.read('cached', 'new'), { count: 5 });
});

test('cached responses expire according to the requesting widget policy', async () => {
  const worker = createWorker({
    clients: new Map([
      ['long', widget(3600)],
      ['short', widget(1)],
    ]),
  });
  assert.deepEqual(await worker.read('long'), { count: 1 });
  worker.advance(999);
  assert.deepEqual(await worker.read('short'), { count: 1 });
  worker.advance(1);
  assert.deepEqual(await worker.read('short'), { count: 2 });
  assert.deepEqual(await worker.read('long'), { count: 2 });
});

test('URL rules override default duration for each widget', async () => {
  const worker = createWorker({
    clients: new Map([
      ['cached', widget(3600)],
      ['rules', widget(3600, [{ urlRegex: '/data$', duration: 0 }])],
    ]),
  });
  await worker.read('cached');
  assert.deepEqual(await worker.read('rules'), { count: 2 });
  assert.deepEqual(await worker.read('rules'), { count: 3 });
  assert.deepEqual(await worker.read('rules', 'asset'), { count: 4 });
  assert.deepEqual(await worker.read('rules', 'asset'), { count: 4 });
});

test('restarted workers retrieve widget policies and can reuse persisted responses', async () => {
  const clients = new Map([
    ['cached', widget(3600)],
    ['fresh', widget(0)],
  ]);
  const first = createWorker({ clients });
  await first.read('cached');
  const restarted = createWorker({ clients, stores: first.stores });
  assert.deepEqual(await restarted.read('cached'), { count: 1 });
  assert.equal(restarted.requests(), 0);
  await restarted.read('fresh');
  await restarted.read('fresh');
  assert.equal(restarted.requests(), 2);
});

test('missing clients and failed policy delivery use the network without caching', async () => {
  const worker = createWorker({
    clients: new Map([
      [
        'closed',
        {
          postMessage() {
            throw new Error('closed');
          },
        },
      ],
    ]),
  });
  await worker.read('');
  await worker.read('missing');
  await worker.read('closed');
  assert.equal(worker.requests(), 3);
  assert.equal(worker.stores.size, 0);
});

test(
  'unresponsive clients have bounded policy lookup and bypass caching',
  { timeout: 5000 },
  async () => {
    const worker = createWorker({
      clients: new Map([['old', { postMessage() {} }]]),
    });
    await worker.read('old');
    assert.equal(worker.requests(), 1);
    assert.equal(worker.stores.size, 0);
  },
);
