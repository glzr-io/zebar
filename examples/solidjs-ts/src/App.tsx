import { ZebarContext } from 'zebar';

import styles from './App.module.css';
import { createEffect } from 'solid-js';
import { createMutable } from 'solid-js/store';

export default async function App(props: { ctx: ZebarContext }) {
  const cpu = await props.ctx.createProvider({ type: 'cpu' });
  const memory = await props.ctx.createProvider({ type: 'memory' });

  // const cpu = createMutable(
  //   await props.ctx.createProvider({ type: 'cpu' }),
  // );
  // const memory = createMutable(
  //   await props.ctx.createProvider({ type: 'memory' }),
  // );

  createEffect(() => {
    console.log('client', cpu);
    console.log('client', cpu.usage);
  });

  return (
    <div class={styles.app}>
      <header class={styles.header}>
        <p>{cpu.usage}</p>
        <p>{memory.usage}</p>
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          class={styles.link}
          href="https://github.com/solidjs/solid"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn Solid
        </a>
      </header>
    </div>
  );
}
