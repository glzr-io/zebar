import { MonitorInfo } from './monitor-info.model';
import { WindowInfo } from './window-info.model';

export interface InitialState {
  args: Record<string, string>;
  env: Record<string, string>;
  currentWindow: WindowInfo;
  currentMonitor?: MonitorInfo;
  primaryMonitor?: MonitorInfo;
  monitors: MonitorInfo[];
}
