import { ZebarContext } from 'zebar';

import styles from './App.module.css';

export async function App(ctx: ZebarContext) {
  const cpu = await ctx.createProvider({ type: 'cpu' });
  const memory = await ctx.createProvider({ type: 'memory' });

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
