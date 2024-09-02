/* @refresh reload */
import { render } from 'solid-js/web';
import { init, ZebarContext } from 'zebar';

// import { App } from './App';
import {
  createEffect,
  createResource,
  createSignal,
  lazy,
  onMount,
  Suspense,
} from 'solid-js';
// import { App2 } from './App2';
import { createStore } from 'solid-js/store';

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?',
  );
}

// const xx = lazy(() => import('./App'));

// init({}, ctx => {
//   // const app = await App({ ctx });
//   // render(() => app, root!);
//   // render(() => <App2 ctx={ctx} />, root!);
//   render(() => <App2 ctx={ctx} />, root!);
// });
// render(() => <App />, root!);
// init({}, async ctx => {
//   render(() => <App2 ctx={ctx} />, root!);
// });

// function App() {
//   const [context, setContext] = createSignal<ZebarContext>();

//   const [aa] = createResource(
//     () => context(),
//     async context => {
//       return {
//         cpu: await context.createProvider({ type: 'cpu' }),
//         memory: await context.createProvider({ type: 'memory' }),
//       };
//     },
//   );

//   init({}, ctx => setContext(ctx));

//   const [providers, setProviders] = createStore();

//   createEffect(async () => {
//     if (context()) {
//       setProviders(
//         // await Promise.all([
//         //   props.ctx.createProvider({ type: 'cpu' }),
//         //   props.ctx.createProvider({ type: 'memory' }),
//         // ]),
//         {
//           cpu: await context().createProvider({ type: 'cpu' }),
//           memory: await context().createProvider({ type: 'memory' }),
//         },
//       );
//       console.log('aaa', providers);
//     }
//   });

//   // return <div>Hello World {aa()?.cpu?.usage}</div>;
//   return <div>Hello World {providers?.cpu?.usage}</div>;
// }

// const ctx = await init();

// const providers = await ctx.createProviders({
//   memory: { type: 'memory' },
//   cpu: { type: 'cpu' },
//   glazewm: { type: 'glazewm' },
//   weather: { type: 'weather' },
// });

// const cpu = await ctx.createProvider({ type: 'cpu' });

// async function bootstrap() {
//   const ctx = await init();
//   const cpu = await ctx.createProvider({ type: 'cpu' });
//   const memory = await ctx.createProvider({ type: 'memory' });
//   // const providers = { cpu, memory };

//   render(() => <App ctx={ctx} providers={providers} />, root!);
// }

// function App(props: {
//   ctx: ZebarContext;
//   providers: { cpu: any; memory: any };
// }) {
// const [providers, setProviders] = createStore(providers);
// providers.onChange(setProviders);

// return <div>Hello World {providers.cpu.usage}</div>;
// }

const ctx = await init();

// const [memory, cpu, glazewm, weather] = await ctx.createProviders([
//   { type: 'memory' },
//   { type: 'cpu' },
//   { type: 'glazewm' },
//   { type: 'weather' },
// ]);
// const battery = await ctx.createProvider({ type: 'battery' });
const cpu = await ctx.createProvider({ type: 'cpu' });

render(() => <App />, root!);

function App() {
  // const [memory, setMemory] = createStore(memory);
  // const [cpu, setCpu] = createStore(cpu);
  // const [glazewm, setGlazewm] = createStore(glazewm);
  // const [weather, setWeather] = createStore(weather);

  // memory.onChange(setMemory);
  // cpu.onChange(setCpu);
  // glazewm.onChange(setGlazewm);
  // weather.onChange(setWeather);

  const [providers, setProviders] = createStore({
    // battery,
    cpu,
  });

  // battery.onChange(battery => setProviders({ battery }));
  //@ts-ignore
  cpu.onChange(cpu => setProviders({ cpu }));

  return <div>Hello World {providers.cpu.usage}</div>;
}
