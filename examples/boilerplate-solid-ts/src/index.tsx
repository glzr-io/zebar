/* @refresh reload */
import './index.css';
import { render } from 'solid-js/web';
import { createStore } from 'solid-js/store';
import * as zebar from 'zebar';
import { createSignal } from 'solid-js';

const providers = zebar.createProviderGroup({
  cpu: { type: 'cpu' },
  focusedWindow: { type: 'focusedWindow' },
  battery: { type: 'battery' },
  memory: { type: 'memory' },
  weather: { type: 'weather' },
});

render(() => <App />, document.getElementById('root')!);

function App() {
  const [output, setOutput] = createStore(providers.outputMap);
  const [iconUrl, setIconUrl] = createSignal<string | null>(null);

  providers.onOutput(outputMap => {
    setOutput(outputMap);

    const icon = outputMap.focusedWindow?.icon;
    if (icon) {
      console.log('Icon data:', icon);
      try {
        // Ensure the base64 string is correctly formatted
        const binary = atob(icon);
        console.log('Binary data length:', binary.length);
        const array = new Uint8Array(binary.length);
        for (let i = 0; i < binary.length; i++) {
          array[i] = binary.charCodeAt(i);
        }
        const blob = new Blob([array], { type: 'image/png' });
        const url = URL.createObjectURL(blob);
        console.log('Generated URL:', url);
        setIconUrl(url);
      } catch (error) {
        console.error('Error decoding icon data:', error);
        setIconUrl(null);
      }
    } else {
      console.log('No icon data available');
      setIconUrl(null);
    }
  });

  return (
    <div class="app">
      <div class="chip">CPU usage: {output.cpu?.usage}</div>
      <div class="chip">Focused window: {output.focusedWindow?.title}</div>
      {iconUrl() && <img src={iconUrl()!} alt="icon" />}
      <div class="chip">
        Battery charge: {output.battery?.chargePercent}
      </div>
      <div class="chip">Memory usage: {output.memory?.usage}</div>
      <div class="chip">Weather temp: {output.weather?.celsiusTemp}</div>
    </div>
  );
}
