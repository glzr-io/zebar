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
  shellExec,
  shellSpawn,
  shellWrite,
  shellKill,
};

export type ProviderFunction = AudioFunction | MediaFunction;

export interface AudioFunction {
  type: 'audio';
  function: {
    name: 'set_volume';
    args: {
      volume: number;
      deviceId?: string;
    };
  };
}

export interface MediaFunction {
  type: 'media';
  function: {
    name: 'play' | 'pause' | 'toggle_play_pause' | 'next' | 'previous';
    args: {
      sessionId?: string;
    };
  };
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

function callProviderFunction(
  configHash: string,
  fn: ProviderFunction,
): Promise<void> {
  return invoke<void>('call_provider_function', {
    configHash,
    function: fn,
  });
}

function setAlwaysOnTop(): Promise<void> {
  return invoke<void>('set_always_on_top');
}

function setSkipTaskbar(skip: boolean): Promise<void> {
  return invoke<void>('set_skip_taskbar', { skip });
}

function shellExec<TOutput extends string | Uint8Array = string>(
  program: string,
  args: string | string[] = [],
  options: ShellCommandOptions = {},
): Promise<ShellExecOutput<TOutput>> {
  return invoke<ShellExecOutput<TOutput>>('shell_exec', {
    program,
    args,
    options,
  });
}

function shellSpawn(
  program: string,
  args: string | string[] = [],
  options: ShellCommandOptions = {},
): Promise<number> {
  return invoke<number>('shell_spawn', { program, args, options });
}

function shellWrite(
  processId: number,
  buffer: string | Uint8Array,
): Promise<void> {
  return invoke<void>('shell_write', { processId, buffer });
}

function shellKill(processId: number): Promise<void> {
  return invoke<void>('shell_kill', { processId });
}

export interface ShellCommandOptions {
  /**
   * Current working directory.
   */
  cwd?: string;

  /**
   * Environment variables.
   */
  env?: Record<string, string> | null;

  /**
   * Clear the environment variables of the spawned process.
   */
  clearEnv?: boolean;

  /**
   * Character encoding for stdout/stderr.
   *
   * Defaults to `utf-8` (string). Use `raw` to return raw bytes
   * (`Uint8Array`).
   */
  encoding?: ShellOutputEncoding;
}

export type ShellOutputEncoding =
  | 'raw'
  | 'utf-8'
  | 'utf-16'
  | 'gbk'
  | 'gb18030'
  | 'big5'
  | 'euc-jp'
  | 'euc-kr'
  | 'iso-2022-jp'
  | 'shift-jis';

export interface ShellExecOutput<
  TOutput extends string | Uint8Array = string,
> {
  code: number | null;
  signal: number | null;
  stdout: TOutput;
  stderr: TOutput;
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
