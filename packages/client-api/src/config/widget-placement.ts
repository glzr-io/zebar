import type { MonitorSelection } from './monitor-selection';
import type { DockConfig } from './dock-config';

export type WidgetPlacement = {
  anchor:
    | 'top_left'
    | 'top_center'
    | 'top_right'
    | 'center_left'
    | 'center'
    | 'center_right'
    | 'bottom_left'
    | 'bottom_center'
    | 'bottom_right';
  offsetX: string;
  offsetY: string;
  width: string;
  height: string;
  monitorSelection: MonitorSelection;
  dockToEdge: DockConfig;
};
