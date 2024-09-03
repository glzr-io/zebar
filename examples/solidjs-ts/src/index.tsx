/* @refresh reload */
import './index.css';
import { render } from 'solid-js/web';
import { createStore } from 'solid-js/store';
import { init } from 'zebar';

const zebarCtx = await init();
const glazewm = await zebarCtx.createProvider({ type: 'glazewm' });
const cpu = await zebarCtx.createProvider({ type: 'cpu' });
const battery = await zebarCtx.createProvider({ type: 'battery' });
const memory = await zebarCtx.createProvider({ type: 'memory' });
const weather = await zebarCtx.createProvider({ type: 'weather' });

render(() => <App />, document.getElementById('root')!);

function App() {
  const [store, setStore] = createStore({
    glazewm: glazewm.value,
    cpu: cpu.value,
    battery: battery.value,
    memory: memory.value,
    weather: weather.value,
  });

  glazewm.onValue(glazewm => setStore({ glazewm }));
  cpu.onValue(cpu => setStore({ cpu }));
  battery.onValue(battery => setStore({ battery }));
  memory.onValue(memory => setStore({ memory }));
  weather.onValue(weather => setStore({ weather }));

  return (
    <div>
      glazewm: {JSON.stringify(store.glazewm)}
      cpu: {store.cpu.usage}
      battery: {store.battery?.chargePercent}
      memory: {store.memory.usage}
      weather temp: {store.weather.celsiusTemp}
      weather status: {store.weather.status}
    </div>
  );
}
