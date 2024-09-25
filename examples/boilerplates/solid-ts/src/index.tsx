/* @refresh reload */
import './index.css';
import { render } from 'solid-js/web';
import { createStore } from 'solid-js/store';
import * as zebar from 'zebar';

// import { init } from 'zebar';
// const zebarCtx = await init();

// zebarCtx.widgets.start('./starter/vanilla');
// zebarCtx.startWidget('./starter/vanilla');
// zebarCtx.currentInstance.tauri.setZOrder('alwaysOnTop');

// zebarCtx.instanceId;
// zebarCtx.config;
// zebarCtx.currentWidget.tauriWindow.getPosition();
// zebarCtx.setZOrder('always_on_top');

const providers = await zebar.createProviderGroup({
  cpu: { type: 'cpu' },
  battery: { type: 'battery' },
  memory: { type: 'memory' },
  weather: { type: 'weather' },
  keyboard: { type: 'keyboard' },
});

const monitor = zebar.currentMonitor();
const window = zebar.currentWindow();
const widget = zebar.currentWidget();

render(() => <App />, document.getElementById('root')!);

function App() {
  const [output, setOutput] = createStore(providers.outputMap);

  providers.onOutput(outputMap => setOutput(outputMap));

  return (
    <div class="app">
      <div class="chip">Keyboard: {output.keyboard.layout}</div>
      <div class="chip">CPU usage: {output.cpu.usage}</div>
      <div class="chip">
        Battery charge: {output.battery?.chargePercent}
      </div>
      <div class="chip">Memory usage: {output.memory.usage}</div>
      <div class="chip">Weather temp: {output.weather?.celsiusTemp}</div>
    </div>
  );
}
