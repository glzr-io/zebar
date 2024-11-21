import {
  type InvokeArgs,
  invoke as tauriInvoke,
} from '@tauri-apps/api/core';

import { createLogger } from '../utils';
import type { ProviderConfig } from '~/providers';
import type { WidgetPlacement } from '~/config';

const logger = createLogger('desktop-commands');

export const desktopCommands = {
  startWidget,
  startPreset,
  listenProvider,
  unlistenProvider,
  callProviderFunction,
  setAlwaysOnTop,
  setSkipTaskbar,
};

export type ProviderFunction = MediaFunction;

export interface MediaFunction {
  type: 'media';
  function: 'play' | 'pause' | 'toggle_play_pause' | 'next' | 'previous';
}

function startWidget(
  configPath: string,
  placement: WidgetPlacement,
): Promise<void> {
  return invoke<void>('start_widget', { configPath, placement });
}

function startPreset(
  configPath: string,
  presetName: string,
): Promise<void> {
  return invoke<void>('start_preset', { configPath, presetName });
}

function listenProvider(args: {
  configHash: string;
  config: ProviderConfig;
}): Promise<void> {
  return invoke<void>('listen_provider', args);
}

function unlistenProvider(configHash: string): Promise<void> {
  return invoke<void>('unlisten_provider', { configHash });
}

function callProviderFunction(args: {
  configHash: string;
  function: ProviderFunction;
}): Promise<void> {
  return invoke<void>('call_provider_function', args);
}

function setAlwaysOnTop(): Promise<void> {
  return invoke<void>('set_always_on_top');
}

function setSkipTaskbar(skip: boolean): Promise<void> {
  return invoke<void>('set_skip_taskbar', { skip });
}

/**
 * Invoke a Tauri command with logging and error handling.
 */
async function invoke<T>(command: string, args?: InvokeArgs): Promise<T> {
  logger.info(`Calling '${command}' with args:`, args ?? {});

  try {
    const response = await tauriInvoke<T>(command, args);
    logger.info(`Response for calling '${command}':`, response);

    return response;
  } catch (err) {
    logger.error(`Command '${command}' failed: ${err}`);
    throw new Error(`Command '${command}' failed: ${err}`);
  }
}
