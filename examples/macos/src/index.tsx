/* @refresh reload */
import './index.css';
import { render } from 'solid-js/web';
import { createStore } from 'solid-js/store';
import * as zebar from 'zebar';
import { createSignal } from 'solid-js';

const providers = zebar.createProviderGroup({
  window: { type: 'window', refreshInterval: 1500 },
  cpu: { type: 'cpu', refreshInterval: 5000  },
  memory: { type: 'memory', refreshInterval: 5000  },
  audio: { type: 'audio' },
  systray: { type: 'systray', refreshInterval: 5000 },
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
        <ul id="dropdown">
          <li onClick={() => openWindowsSettings()}><button>About This PC</button></li>
          <li onClick={() => openSystemSettings()}><button>System Preferences</button></li>
          <li onClick={() => openMicrosoftStore()}><button>App Store</button></li>
          {showSleepOptions() ? (
            <li class="act">
              <button onClick={() => sleepWindows()}>Sleep ({countdown()}s)</button>
              <button onClick={() => resetAllOptions()}>Cancel</button>
            </li>
          ) : (
            <li onClick={() => { resetAllOptions(); setShowSleepOptions(true); startCountdown(sleepWindows); }}>
              <button>Sleep</button>
            </li>
          )}
          {showShutdownOptions() ? (
            <li class="act">
              <button onClick={() => shutdownWindows()}>Shut Down ({countdown()}s)</button>
              <button onClick={() => resetAllOptions()}>Cancel</button>
            </li>
          ) : (
            <li onClick={() => { resetAllOptions(); setShowShutdownOptions(true); startCountdown(shutdownWindows); }}>
              <button>Shut Down</button>
            </li>
          )}
          {showRestartOptions() ? (
            <li class="act">
              <button onClick={() => restartWindows()}>Restart ({countdown()}s)</button>
              <button onClick={() => resetAllOptions()}>Cancel</button>
            </li>
          ) : (
            <li onClick={() => { resetAllOptions(); setShowRestartOptions(true); startCountdown(restartWindows); }}>
              <button>Restart</button>
            </li>
          )}
          {showLogOutOptions() ? (
            <li class="act">
              <button onClick={() => logOut()}>Log Out ({countdown()}s)</button>
              <button onClick={() => resetAllOptions()}>Cancel</button>
            </li>
          ) : (
            <li onClick={() => { resetAllOptions(); setShowLogOutOptions(true); startCountdown(logOut); }}>
              <button>Log Out</button>
            </li>
          )}
        </ul>
        <ul>
          <li>
            <button class="app"
              onClick={() => {
                if (output.window?.title === 'File Explorer') {
                  openFileExplorer();
                }
              }}
            >
              {output.window?.title || 'File Explorer'}
            </button>
          </li>
        </ul>
      </div>

      <div class="right">
        <ul>
          {output.cpu && (
            <li>
              <i class="nf nf-oct-cpu"></i>
              <span class={output.cpu.usage > 85 ? 'high-usage' : ''}>
                {Math.round(output.cpu.usage)}%
              </span>
            </li>
          )}
          {output.memory && (
            <li>
              <i class="nf nf-fae-chip"></i>
              {Math.round(output.memory.usage)}%
            </li>
          )}
          {output.audio?.defaultPlaybackDevice && (
            <li>
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
            </li>
          )}
          {output.systray && (
            <li>
              <ul>
                {output.systray.icons
                  .filter(icon => !icon.tooltip?.toLowerCase().includes('speakers')) // Exclude icons where tooltip includes "Speakers"
                  .slice() // Create a copy of the array to avoid mutating the original
                  .sort((a, b) => {
                    const priorityKeywords = ["cpu core", "gpu"];
                    const aPriority = priorityKeywords.findIndex(keyword =>
                      a.tooltip?.toLowerCase().includes(keyword)
                    );
                    const bPriority = priorityKeywords.findIndex(keyword =>
                      b.tooltip?.toLowerCase().includes(keyword)
                    );

                    if (aPriority !== -1 && bPriority === -1) return -1; // `a` is a priority
                    if (aPriority === -1 && bPriority !== -1) return 1;  // `b` is a priority
                    if (aPriority !== -1 && bPriority !== -1) return aPriority - bPriority; // Both are priorities
                    return 0; // Neither is a priority
                  })
                  .map(icon => (
                    <li>
                      <input
                        type="image"
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
                    </li>
                  ))}
              </ul>
            </li>
          )}
          {output.date && (
            <li>
              {output.date?.formatted}
            </li>
          )}
        </ul>
      </div>
    </div>
  );
}
