/* @refresh reload */
import './index.css';
import { render } from 'solid-js/web';
import { createStore } from 'solid-js/store';
import { init } from 'zebar';

const zebarCtx = await init();
const cpu = await zebarCtx.createProvider({ type: 'cpu' });

render(() => <App />, document.getElementById('root')!);

function App() {
  const [store, setStore] = createStore({
    cpu: cpu.value,
  });

  cpu.onValue(cpu => setStore({ cpu }));

  return <div>Hello World {store.cpu.usage}</div>;
}
