export interface InitialState {
  currentWindow: WindowInfo;
  currentMonitor?: MonitorInfo;
  primaryMonitor?: MonitorInfo;
  monitors: MonitorInfo[];
}

export interface MonitorInfo {
  name: string;
  x: number;
  y: number;
  width: number;
  height: number;
  scaleFactor: number;
}

export interface WindowInfo {
  x: number;
  y: number;
  width: number;
  height: number;
  scaleFactor: number;
}
