import { createEffect, createResource } from 'solid-js';

import {
  GlobalConfigSchema,
  UserConfig,
  WindowConfig,
  buildStyles,
  getConfigVariables,
  getUserConfig,
  parseConfigSection,
} from './user-config';
import { ElementContext, createContextStore } from './context';
import { createTemplateEngine } from './template-engine';
import {
  listenProvider,
  onProviderEmit,
  setWindowPosition,
  setWindowStyles,
  unlistenProvider,
} from './desktop';
import { simpleHash } from './utils';

export async function initAsync() {
  // TODO: Promisify `init`.
}

export function init(callback: (context: ElementContext) => void) {
  const [config] = getUserConfig();
  const [configVariables] = getConfigVariables();
  const templateEngine = createTemplateEngine();

  const context = createContextStore(config, configVariables, templateEngine);

  const [globalConfig] = createResource(
    () => config(),
    config =>
      parseConfigSection(
        templateEngine,
        (config as UserConfig).global,
        GlobalConfigSchema.strip(),
        {},
      ),
  );

  // Dynamically create <style> tag and append it to <head>.
  createEffect(async () => {
    if (globalConfig() && context.store.hasInitialized) {
      const styleElement = document.createElement('style');
      document.head.appendChild(styleElement);
      styleElement.innerHTML = await buildStyles(
        globalConfig()!,
        context.store.value!,
      );

      return () => document.head.removeChild(styleElement);
    }
  });

  const cpuOptions = { type: 'cpu', refresh_interval_ms: 5000 };
  const cpuOptionsHash = simpleHash(cpuOptions);
  listenProvider({
    optionsHash: cpuOptionsHash,
    options: cpuOptions,
    trackedAccess: [],
  });

  const networkOptions = { type: 'network', refresh_interval_ms: 5000 };
  const networkOptionsHash = simpleHash(networkOptions);
  listenProvider({
    optionsHash: networkOptionsHash,
    options: networkOptions,
    trackedAccess: [],
  });
  onProviderEmit(networkOptionsHash, payload => console.log('provider emit'));
  setTimeout(() => {
    console.log('aaa');
    unlistenProvider(networkOptionsHash);

    console.log('bbb');
  }, 6000);
  const batteryOptions = { type: 'battery', refresh_interval_ms: 5000 };
  const batteryOptionsHash = simpleHash(batteryOptions);
  listenProvider({
    optionsHash: batteryOptionsHash,
    options: batteryOptions,
    trackedAccess: [],
  });
  const hostOptions = { type: 'host', refresh_interval_ms: 5000 };
  const hostOptionsHash = simpleHash(hostOptions);
  listenProvider({
    optionsHash: hostOptionsHash,
    options: hostOptions,
    trackedAccess: [],
  });

  // Set window position based on config values.
  createEffect(async () => {
    if (globalConfig() && context.store.hasInitialized) {
      const windowConfig = context.store.value!.parsedConfig as WindowConfig;

      await setWindowPosition({
        x: windowConfig.position_x,
        y: windowConfig.position_y,
        width: windowConfig.width,
        height: windowConfig.height,
      });

      await setWindowStyles({
        alwaysOnTop: windowConfig.always_on_top,
        showInTaskbar: windowConfig.show_in_taskbar,
        resizable: windowConfig.resizable,
      });
    }
  });

  // Invoke callback passed to `init`.
  createEffect(() => {
    if (context.store.hasInitialized) {
      callback(context.store.value!);
    }
  });
}
