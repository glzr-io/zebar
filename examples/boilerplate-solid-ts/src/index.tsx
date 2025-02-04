/* @refresh reload */
import './index.css';
import { For, render } from 'solid-js/web';
import { createStore } from 'solid-js/store';
import * as zebar from 'zebar';

const providers = zebar.createProviderGroup({
  audio: { type: 'audio' },
  cpu: { type: 'cpu' },
  battery: { type: 'battery' },
  memory: { type: 'memory' },
  weather: { type: 'weather' },
  media: { type: 'media' },
  systray: { type: 'systray' },
});

render(() => <App />, document.getElementById('root')!);

function App() {
  const [output, setOutput] = createStore(providers.outputMap);

  providers.onOutput(outputMap => setOutput(outputMap));

  return (
    <div class="app">
      {output.audio?.defaultPlaybackDevice && (
        <div class="chip">
          {output.audio.defaultPlaybackDevice.name}-
          {output.audio.defaultPlaybackDevice.volume}
          <input
            type="range"
            min="0"
            max="100"
            step="2"
            value={output.audio.defaultPlaybackDevice.volume}
            onChange={e => output.audio.setVolume(e.target.valueAsNumber)}
          />
        </div>
      )}
      {output.media?.currentSession && (
        <div class="chip">
          Media: {output.media.currentSession.title}-
          {output.media.currentSession.artist}
          <button onClick={() => output.media?.togglePlayPause()}>
            ⏯
          </button>
        </div>
      )}
      {output.cpu && <div class="chip">CPU usage: {output.cpu.usage}</div>}
      {output.battery && (
        <div class="chip">
          Battery charge: {output.battery.chargePercent}
        </div>
      )}
      {output.memory && (
        <div class="chip">Memory usage: {output.memory.usage}</div>
      )}
      {output.weather && (
        <div class="chip">Weather temp: {output.weather.celsiusTemp}</div>
      )}
      {output.systray && (
        <div class="chip">
          <For each={output.systray.icons}>
            {icon => (
              <img
                class="systray-icon"
                src={icon.iconUrl}
                onClick={e => {
                  e.preventDefault();
                  output.systray.onLeftClick(icon.id);
                }}
                onContextMenu={e => {
                  e.preventDefault();
                  output.systray.onRightClick(icon.id);
                }}
                onMouseEnter={e => {
                  e.preventDefault();
                  output.systray.onHoverEnter(icon.id);
                }}
                onMouseLeave={e => {
                  e.preventDefault();
                  output.systray.onHoverLeave(icon.id);
                }}
                onMouseMove={e => {
                  e.preventDefault();
                  output.systray.onHoverMove(icon.id);
                }}
              />
            )}
          </For>
        </div>
      )}
    </div>
  );
}
