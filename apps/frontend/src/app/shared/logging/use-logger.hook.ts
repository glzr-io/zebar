import { memoize } from '../utils';
import { LogLevel } from './log-level.enum';

type LogMethod = 'log' | 'warn' | 'error';

// TODO: Get a minimum log level from environment.
export const useLogger = memoize((section: string) => {
  function log(consoleLogMethod: LogMethod, message: string, ...data: any[]) {
    const date = new Date();
    const timestamp = `${date.getHours()}:${date.getMinutes()}:${date.getSeconds()}:${date.getMilliseconds()}`;

    // Clone data to avoid reference changes in Chrome console.
    // TODO: This needs to be improved.
    const clonedData = data.map(data => {
      return data === null || data === undefined ? data : structuredClone(data);
    });

    console[consoleLogMethod](
      `${timestamp} [${section}] ${message}`,
      ...clonedData,
    );
  }

  function debug(message: string, ...data: any) {
    if (shouldLog(LogLevel.DEBUG)) log('log', message, ...data);
  }

  function info(message: string, ...data: any) {
    if (shouldLog(LogLevel.INFO)) log('log', message, ...data);
  }

  function warn(message: string, ...data: any) {
    if (shouldLog(LogLevel.WARNING)) log('warn', message, ...data);
  }

  function error(message: string, ...data: any) {
    if (shouldLog(LogLevel.ERROR)) log('error', message, ...data);
  }

  function shouldLog(logLevel: LogLevel): boolean {
    return true;
  }

  return {
    debug,
    info,
    warn,
    error,
  };
});
