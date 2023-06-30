import { Logger } from './logger';

export function useLogger(section: string) {
  return new Logger(section);
}
