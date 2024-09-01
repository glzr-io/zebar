import { ZebarContext } from 'zebar';

import styles from './App.module.css';
import {
  Component,
  createEffect,
  createResource,
  onCleanup,
  onMount,
  Show,
  Suspense,
} from 'solid-js';
import { createMutable, createStore } from 'solid-js/store';

export function App2(props: { ctx: ZebarContext }) {
  const [cpu] = createResource(() =>
    props.ctx.createProvider({ type: 'cpu' }),
  );

  const [memory] = createResource(() =>
    props.ctx.createProvider({ type: 'memory' }),
  );

  // const cpu = createMutable(
  //   await props.ctx.createProvider({ type: 'cpu' }),
  // );
  // const memory = createMutable(
  //   await props.ctx.createProvider({ type: 'memory' }),
  // );

  // const [providers] = createResource(async () => {
  //   return {
  //     cpu: await props.ctx.createProvider({ type: 'cpu' }),
  //     memory: await props.ctx.createProvider({ type: 'memory' }),
  //   };
  // });

  // createEffect(() => {
  //   console.log('client', providers());
  //   console.log('client', providers()?.cpu);
  // });

  // const [providers, setProviders] = createStore([]);

  // onMount(async () => {
  //   setProviders(
  //     await Promise.all([
  //       props.ctx.createProvider({ type: 'cpu' }),
  //       props.ctx.createProvider({ type: 'memory' }),
  //     ]),
  //   );
  // });

  createEffect(() => {
    console.log('client', cpu());
    console.log('client', cpu()?.usage);
  });

  onCleanup(() => {
    console.log('cleanup');
  });

  return (
    <div class={styles.app}>
      <Show when={cpu()}>
        <p>{cpu().usage}</p>
      </Show>
    </div>
  );
  // return (
  //   <div class={styles.app}>
  //     <p>{providers[0]?.usage}</p>
  //     <p>{providers[0]?.usage}</p>
  //   </div>
  //   // <Show when={providers}>
  //   //   {(providers) => {
  //   //     <div class={styles.app}>
  //   //       <p>{cpu.usage}</p>
  //   //       <p>{memory.usage}</p>
  //   //     </div>;
  //   //   }}
  //   // </Show>
  // );
  // const [providers, setProviders] = createStore({
  //   cpu: null,
  //   memory: null,
  //   loaded: false,
  // });

  // onMount(async () => {
  //   setProviders({
  //     loaded: true,
  //     cpu: await props.ctx.createProvider({ type: 'cpu' }),
  //     memory: await props.ctx.createProvider({ type: 'memory' }),
  //   });
  // });

  // return (
  //   <Show when={providers.loaded}>
  //     <div class={styles.app}>
  //       <p>{providers.cpu?.usage}</p>
  //       <p>{providers.memory?.usage}</p>
  //     </div>
  //   </Show>
  // );
}
