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

console.log('xxxxxxxxx', await zebar.shellExec('echo', ['hdfldas']));

const test = await zebar.shellExec('ping', [
  '127.0.0.1',
  '-n',
  '10',
  '-w',
  '3000',
]);
console.log('test', test);
const git = await zebar.shellSpawn('ping', [
  '127.0.0.1',
  '-n',
  '10',
  '-w',
  '3000',
]);
git.onStdout(output => console.log('stdout', output));
git.onStderr(output => console.log('stderr', output));
git.onExit(output => console.log('exit', output));
console.log('xxxxxxxxx', await zebar.shellExec('node', ['fjdisa']));
// console.log('xxxxxxxxx', await zebar.shellExec('code', ['.']));
// console.log(
//   'xxxxxxxxx',
//   await zebar.shellExec(
//     'C:/Users/larsb/AppData/Local/Programs/cursor/resources/app/bin/code',
//   ),
// );
// console.log('xxxxxxxxx', await zebar.shellExec('code', ['.']));

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
      <div class="chip">
        Media: {output.media?.currentSession?.title}-
        {output.media?.currentSession?.artist}
        <button onClick={() => output.media?.togglePlayPause()}>⏯</button>
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
