/* @refresh reload */
import './index.css';
import { render } from 'solid-js/web';
import { createStore } from 'solid-js/store';
import * as zebar from 'zebar';

const providers = zebar.createProviderGroup({
  audio: { type: 'audio' },
  cpu: { type: 'cpu' },
  battery: { type: 'battery' },
  memory: { type: 'memory' },
  weather: { type: 'weather' },
  media: { type: 'media' },
});

render(() => <App />, document.getElementById('root')!);

function App() {
  const [output, setOutput] = createStore(providers.outputMap);

  providers.onOutput(outputMap => setOutput(outputMap));

  return (
    <div class="app">
      <div class="chip">
        {output.audio?.defaultPlaybackDevice?.name} -
        {output.audio?.defaultPlaybackDevice?.volume}
      </div>
      <div class="chip">
        Media: {output.media?.currentSession?.title} -
        {output.media?.currentSession?.artist}
        <button onClick={() => output.media?.togglePlayPause()}>
          ⏯
        </button>
      </div>
      <div class="chip">CPU usage: {output.cpu?.usage}</div>
      <div class="chip">
        Battery charge: {output.battery?.chargePercent}
      </div>
      <div class="chip">Memory usage: {output.memory?.usage}</div>
      <div class="chip">Weather temp: {output.weather?.celsiusTemp}</div>
    </div>
  );
}
