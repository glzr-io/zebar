/* @refresh reload */
import './index.css';
import { render } from 'solid-js/web';
import { createStore } from 'solid-js/store';
import { init } from 'zebar';

const zebarCtx = await init({ includeDefaultCss: true });
const [cpu, battery, memory, weather] = await Promise.all([
  zebarCtx.createProvider({ type: 'cpu' }),
  zebarCtx.createProvider({ type: 'battery' }),
  zebarCtx.createProvider({ type: 'memory' }),
  zebarCtx.createProvider({ type: 'weather' }),
]);

render(() => <App />, document.getElementById('root')!);

function App() {
  const [store, setStore] = createStore({
    cpu: cpu.output,
    battery: battery.output,
    memory: memory.output,
    weather: weather.output,
  });

  cpu.onOutput(cpu => setStore({ cpu }));
  battery.onOutput(battery => setStore({ battery }));
  memory.onOutput(memory => setStore({ memory }));
  weather.onOutput(weather => setStore({ weather }));

  return (
    <div>
      cpu: {store.cpu.usage}
      battery: {store.battery?.chargePercent}
      memory: {store.memory.usage}
      weather temp: {store.weather.celsiusTemp}
      weather status: {store.weather.status}
    </div>
  );
}
