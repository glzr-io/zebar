import { listen, type Event } from '@tauri-apps/api/event';
import {
  desktopCommands,
  type ShellCommandOptions,
  type ShellExecuteOutput,
} from './desktop-commands';

interface ShellEmission {
  pid: number;
  event: ShellEvent;
}

type ShellEvent<T extends string | Uint8Array = string> =
  | {
      type: 'stdout';
      data: T;
    }
  | {
      type: 'stderr';
      data: T;
    }
  | {
      type: 'error';
      data: string;
    }
  | {
      type: 'terminated';
      data: {
        exitCode: number | null;
        signal: number | null;
      };
    };

/**
 * Executes a shell command and waits for completion.
 *
 * @param {string} command - Path to program executable, or program name
 * (if in $PATH).
 * @param {string | string[]} args - Arguments to pass to the program.
 * @param {Object} options - Spawn options (optional).
 * @throws - If the command fails to execute.
 */
export async function shellExec<
  TOutput extends string | Uint8Array = string,
>(
  program: string,
  args?: string | string[],
  options?: ShellCommandOptions,
): Promise<ShellExecuteOutput<TOutput>> {
  return await desktopCommands.shellExecute(program, args, options);
}

/**
 * Starts a shell command without waiting for completion. Allows for
 * interaction with the spawned process, such as sending input and killing
 * the process.
 *
 * @param {string} command - Path to program executable, or program name
 * (if in $PATH).
 * @param {string | string[]} args - Arguments to pass to the program.
 * @param {Object} options - Spawn options (optional).
 */
export async function shellSpawn<
  TOutput extends string | Uint8Array = string,
>(
  program: string,
  args?: string | string[],
  options?: ShellCommandOptions,
): Promise<ShellProcess<TOutput>> {
  const processId = await desktopCommands.shellSpawn(
    program,
    args,
    options,
  );

  const stdoutCallbacks: ((data: TOutput) => void)[] = [];
  const stderrCallbacks: ((data: TOutput) => void)[] = [];
  const errorCallbacks: ((data: string) => void)[] = [];
  const exitCallbacks: ((data: {
    exitCode: number | null;
    signal: number | null;
  }) => void)[] = [];

  const unlistenEvents = await listen(
    'shell-emit',
    (event: Event<ShellEmission>) => {
      if (event.payload.pid === processId) {
        const shellEvent = event.payload.event;

        switch (shellEvent.type) {
          case 'stdout':
            stdoutCallbacks.forEach(callback =>
              callback(shellEvent.data as TOutput),
            );
            break;
          case 'stderr':
            stderrCallbacks.forEach(callback =>
              callback(shellEvent.data as TOutput),
            );
            break;
          case 'error':
            errorCallbacks.forEach(callback => callback(shellEvent.data));
            break;
          case 'terminated':
            exitCallbacks.forEach(callback => callback(shellEvent.data));
            unlistenEvents();
            break;
        }
      }
    },
  );

  return {
    processId,
    onStdout: callback => stdoutCallbacks.push(callback),
    onStderr: callback => stderrCallbacks.push(callback),
    onExit: callback => exitCallbacks.push(callback),
    kill: () => desktopCommands.shellKill(processId),
    write: data => desktopCommands.shellWrite(processId, data),
  };
}

export interface ShellProcess<
  TOutput extends string | Uint8Array = string,
> {
  processId: number;
  onStdout: (callback: (line: TOutput) => void) => void;
  onStderr: (callback: (line: TOutput) => void) => void;
  onExit: (
    callback: (status: {
      exitCode: number | null;
      signal: number | null;
    }) => void,
  ) => void;
  kill: () => void;
  write: (data: string | Uint8Array) => void;
}
