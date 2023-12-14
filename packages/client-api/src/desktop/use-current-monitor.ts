import {
  availableMonitors as getAvailableMonitors,
  currentMonitor as getCurrentMonitor,
  primaryMonitor as getPrimaryMonitor,
} from '@tauri-apps/api/window';
import { createResource, createSignal } from 'solid-js';
import { MonitorInfo } from './initial-state.model';
import { createStore } from 'solid-js/store';

// TODO: Should probably be changed to `useMonitors`.
export async function useCurrentMonitor() {
  const currentMonitor = createResource(async () => {
    const monitor = await getCurrentMonitor();

    if (!monitor) {
      throw new Error('Unable to detect current monitor.');
    }

    return {
      x: monitor.position.x,
      y: monitor.position.y,
      width: monitor.size.width,
      height: monitor.size.height,
    };
  });

  // TODO: On display setting changes, refetch.

  return currentMonitor;
}

const [monitors, setMonitors] = createStore({
  current: null,
  primary: null,
  secondary: null,
  all: null,
});

const [current, setCurrent] = createSignal<MonitorInfo>();
const [primary, setPrimary] = createSignal<MonitorInfo>();
const [secondary, setSecondary] = createSignal<MonitorInfo[]>();
const [all, setAll] = createSignal<MonitorInfo[]>();

export async function getMonitors() {
  const [currentMonitor, primaryMonitor, allMonitors] = await Promise.all([
    getCurrentMonitor(),
    getPrimaryMonitor(),
    getAvailableMonitors(),
  ]);

  // TODO: On display setting changes, refetch.

  return currentMonitor;
}

let fullMonitorInfoPromise: Promise<any> | null = null;

async function getFullMonitorInfo() {
  const [currentMonitor, primaryMonitor, allMonitors] = await Promise.all([
    getCurrentMonitor(),
    getPrimaryMonitor(),
    getAvailableMonitors(),
  ]);
}
