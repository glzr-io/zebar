/* @refresh reload */
import './index.css';
import { render } from 'solid-js/web';
import { createStore } from 'solid-js/store';
import { init } from 'zebar';

const zebarCtx = await init();

const [cpu, battery, memory, weather] = await Promise.all([
  zebarCtx.createProvider({ type: 'cpu' }),
  zebarCtx.createProvider({ type: 'battery' }),
  zebarCtx.createProvider({ type: 'memory' }),
  zebarCtx.createProvider({ type: 'weather' }),
]);

render(() => <App />, document.getElementById('root')!);

function App() {
  const [outputs, setOutputs] = createStore({
    cpu: cpu.output,
    battery: battery.output,
    memory: memory.output,
    weather: weather.output,
  });

  cpu.onOutput(cpu => setOutputs({ cpu }));
  battery.onOutput(battery => setOutputs({ battery }));
  memory.onOutput(memory => setOutputs({ memory }));
  weather.onOutput(weather => setOutputs({ weather }));

  return (
    <div class="app">
      <div class="chip">CPU usage: {outputs.cpu.usage}</div>
      <div class="chip">
        Battery charge: {outputs.battery?.chargePercent}
      </div>
      <div class="chip">Memory usage: {outputs.memory.usage}</div>
      <div class="chip">Weather temp: {outputs.weather?.celsiusTemp}</div>
    </div>
  );
}
