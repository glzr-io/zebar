import { LogLevel } from './log-level.enum';

type LogMethod = 'log' | 'warn' | 'error';

export class Logger {
  // TODO: Take in a minimum log level as argument.
  constructor(private section: string) {}

  log(consoleLogMethod: LogMethod, message: string, ...data: any[]) {
    const date = new Date();
    const timestamp = `${date.getHours()}:${date.getMinutes()}:${date.getSeconds()}:${date.getMilliseconds()}`;

    // Clone data to avoid reference changes in Chrome console.
    // TODO: This needs to be improved.
    const clonedData = data.map(data => {
      return data === null || data === undefined ? data : structuredClone(data);
    });

    console[consoleLogMethod](
      `${timestamp} [${this.section}] ${message}`,
      ...clonedData,
    );
  }

  debug(message: string, ...data: any) {
    if (this.shouldLog(LogLevel.DEBUG)) this.log('log', message, ...data);
  }

  info(message: string, ...data: any) {
    if (this.shouldLog(LogLevel.INFO)) this.log('log', message, ...data);
  }

  warn(message: string, ...data: any) {
    if (this.shouldLog(LogLevel.WARNING)) this.log('warn', message, ...data);
  }

  error(message: string, ...data: any) {
    if (this.shouldLog(LogLevel.ERROR)) this.log('error', message, ...data);
  }

  shouldLog(logLevel: LogLevel): boolean {
    return true;
  }
}
