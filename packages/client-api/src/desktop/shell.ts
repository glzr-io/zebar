import { listen } from '@tauri-apps/api/event';
import {
  desktopCommands,
  type ShellCommandOptions,
  type ShellExecuteOutput,
} from './desktop-commands';

type ShellEvent = {
  processId: number;
  event: string;
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

  const events = listen('shell-event', (event: Event<ShellEvent>) => {
    if (event.payload.processId === processId) {
      callback(event);
    }
  });

  return {
    processId,
    onStdout: callback => command.stdout.on('data', callback),
    onStderr: callback => command.stderr.on('data', callback),
    onExit: callback =>
      command.on('close', status =>
        callback({ exitCode: status.code ?? 0 }),
      ),
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
