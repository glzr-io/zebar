/* @refresh reload */
import './index.css';
import { render } from 'solid-js/web';
import { createStore } from 'solid-js/store';
import * as zebar from 'zebar';
import { createSignal } from 'solid-js';

const providers = zebar.createProviderGroup({
  window: { type: 'window' },
  cpu: { type: 'cpu' },
  memory: { type: 'memory' },
  audio: { type: 'audio' },
  systray: { type: 'systray' },
  date: { type: 'date', formatting: 'EEE d MMM t' },
});

render(() => <App />, document.getElementById('root')!);

function App() {
  const [output, setOutput] = createStore(providers.outputMap);

  const [showSleepOptions, setShowSleepOptions] = createSignal(false);
  const [showShutdownOptions, setShowShutdownOptions] = createSignal(false);
  const [showRestartOptions, setShowRestartOptions] = createSignal(false);
  const [showLogOutOptions, setShowLogOutOptions] = createSignal(false);
  const [countdown, setCountdown] = createSignal(60);
  let countdownInteval: number | undefined;

  function startCountdown(action: () => void) {
    countdownInteval = setInterval(() => {
      setCountdown(prev => {
        if (prev <= 1) {
          clearInterval(countdownInteval);
          action();
          resetAllOptions();
          return 60;
        }
        return prev - 1;
      });
    }, 1000);
  }
  
  function resetAllOptions() {
    setShowShutdownOptions(false);
    setShowSleepOptions(false);
    setShowRestartOptions(false);
    setShowLogOutOptions(false);
    setCountdown(60);
    if (countdownInteval !== undefined) {
      clearInterval(countdownInteval);
      countdownInteval = undefined;
    }
  }

  providers.onOutput(outputMap => setOutput(outputMap));

  function toggleDropdown() {
    const dropdownMenu = document.getElementById('dropdown');
    const appleIcon = document.querySelector('.logo');
  
    if (dropdownMenu && appleIcon) {
      const isVisible = dropdownMenu.style.display === 'block';
      dropdownMenu.style.display = isVisible ? 'none' : 'block';
  
      // Toggle the 'active' class on the Apple icon
      if (isVisible) {
        appleIcon.classList.remove('active');
      } else {
        appleIcon.classList.add('active');
      }
    }
  }

  async function openWindowsSettings() {
    try {
      const result = await zebar.shellExec('powershell', ['/c', 'start', 'ms-settings:']);
      console.log('Windows Settings opened:', result.stdout);
    } catch (err) {
      console.error('Failed to open Windows Settings:', err);
    }
  }

  async function openSystemSettings() {
    try {
      const result = await zebar.shellExec('powershell', ['/c', 'start', 'ms-settings:system']);
      console.log('System Settings opened:', result.stdout);
    } catch (err) {
      console.error('Failed to open System Settings:', err);
    }
  }

  async function openMicrosoftStore() {
    try {
      const result = await zebar.shellExec('powershell', ['/c', 'start', 'ms-windows-store:']);
      console.log('Microsoft Store opened:', result.stdout);
    } catch (err) {
      console.error('Failed to open Microsoft Store:', err);
    }
  }

  async function sleepWindows() {
    try {
      const result = await zebar.shellExec('rundll32.exe', ['powrprof.dll,SetSuspendState', '0', '1', '0']);
      console.log('Sleep command executed:', result.stdout);
    } catch (err) {
      console.error('Failed to put the system to sleep:', err);
    }
  }

  async function restartWindows() {
    try {
      const result = await zebar.shellExec('shutdown', ['/r', '/t', '0']); // Restart immediately
      console.log('Restart command executed:', result.stdout);
    } catch (err) {
      console.error('Failed to restart:', err);
    }
  }

  async function shutdownWindows() {
    try {
      const result = await zebar.shellExec('shutdown', ['/s', '/t', '0']); // Shut down immediately
      console.log('Shutdown command executed:', result.stdout);
    } catch (err) {
      console.error('Failed to shut down:', err);
    }
  }

  async function logOut() {
    try {
      const result = await zebar.shellExec('shutdown', ['/l']); // Log out
      console.log('Log out command executed:', result.stdout);
    } catch (err) {
      console.error('Failed to log out:', err);
    }
  }

  async function openFileExplorer() {
    try {
      await zebar.shellExec('powershell', ['-Command', 'Start-Process $HOME']);
    } catch (err) {
      console.error('Failed to open File Explorer:', err);
    }
  }

  return (
    <div class="app">
      <div class="left">
        <i class="logo nf nf-fa-windows" onClick={() => toggleDropdown()}></i>
        <div class="menu" id="dropdown">
          <ul>
            <li onClick={() => openWindowsSettings()}><span>About This PC</span></li>
            <hr />
            <li onClick={() => openSystemSettings()}><span>System Preferences</span></li>
            <li onClick={() => openMicrosoftStore()}><span>App Store</span></li>
            <hr />
            {showSleepOptions() ? (
              <li>
                <button onClick={() => resetAllOptions()}>Cancel</button>
                <button onClick={() => sleepWindows()}>Sleep ({countdown()}s)</button>
              </li>
            ) : (
              <li onClick={() => { resetAllOptions(); setShowSleepOptions(true); startCountdown(sleepWindows); }}>
                <span>Sleep</span>
              </li>
            )}
            {showShutdownOptions() ? (
              <li>
                <button onClick={() => resetAllOptions()}>Cancel</button>
                <button onClick={() => shutdownWindows()}>Shut Down ({countdown()}s)</button>
              </li>
            ) : (
              <li onClick={() => { resetAllOptions(); setShowShutdownOptions(true); startCountdown(shutdownWindows); }}>
                <span>Shut Down</span>
              </li>
            )}
            {showRestartOptions() ? (
              <li>
                <button onClick={() => resetAllOptions()}>Cancel</button>
                <button onClick={() => restartWindows()}>Restart ({countdown()}s)</button>
              </li>
            ) : (
              <li onClick={() => { resetAllOptions(); setShowRestartOptions(true); startCountdown(restartWindows); }}>
                <span>Restart</span>
              </li>
            )}
            {showLogOutOptions() ? (
              <li>
                <button onClick={() => resetAllOptions()}>Cancel</button>
                <button onClick={() => logOut()}>Log Out ({countdown()}s)</button>
              </li>
            ) : (
              <li onClick={() => { resetAllOptions(); setShowLogOutOptions(true); startCountdown(logOut); }}>
                <span>Log Out</span>
              </li>
            )}
          </ul>
        </div>
        <div class="menu">
          <ul>
            <li>
              <b
                onClick={() => {
                  if (output.window?.title === 'File Explorer') {
                    openFileExplorer();
                  }
                }}
              >
                {output.window?.title || 'File Explorer'}
              </b>
            </li>
          </ul>
        </div>
      </div>

      <div class="right">
        {output.cpu && (
          <div class="chip">
            <i class="nf nf-oct-cpu"></i>

            <span class={output.cpu.usage > 85 ? 'high-usage' : ''}>
              {Math.round(output.cpu.usage)}%
            </span>
          </div>
        )}

        {output.memory && (
          <div class="chip">
            <i class="nf nf-fae-chip"></i>
            {Math.round(output.memory.usage)}%
          </div>
        )}

        {output.audio?.defaultPlaybackDevice && (
            <div class="chip volume">
            <input
              type="range"
              min="0"
              max="100"
              step="2"
              value={output.audio.defaultPlaybackDevice.volume}
              onChange={(e: Event & { target: HTMLInputElement }) =>
              output.audio.setVolume(e.target.valueAsNumber)
              }
            />
            </div>
        )}

        {output.systray && (
          <div class="chip">
            {output.systray.icons.map(icon => (
              <img
                class="systray-icon"
                src={icon.iconUrl}
                title={icon.tooltip}
                onClick={e => {
                  e.preventDefault();
                  output.systray.onLeftClick(icon.id);
                }}
                onContextMenu={e => {
                  e.preventDefault();
                  output.systray.onRightClick(icon.id);
                }}
              />
            ))}
          </div>
        )}

        <div class="chip clock">{output.date?.formatted}</div>
      </div>
    </div>
  );
}
