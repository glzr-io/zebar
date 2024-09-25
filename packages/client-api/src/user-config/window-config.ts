export interface WidgetInstanceConfig {
  /**
   * Whether to show the window above/below all others.
   */
  zOrder: ZOrder;

  /**
   * Whether the window should be shown in the taskbar.
   */
  shownInTaskbar: boolean;

  /**
   * Whether the window should have resize handles.
   */
  resizable: boolean;

  /**
   * Whether the window is transparent.
   */
  transparent: boolean;

  /**
   * Where to place the window.
   */
  placements: WindowPlacement[];
}

export enum ZOrder {
  ALWAYS_ON_BOTTOM = 'always_on_bottom',
  ALWAYS_ON_TOP = 'always_on_top',
  NORMAL = 'normal',
}

export interface WindowPlacement {
  /**
   * The monitor index to place the window on.
   */
  monitor_index: 0;

  /**
   * TODO: Add description.
   */
  position: WindowPosition;

  /**
   * TODO: Add description.
   */
  offset_x: 20;

  /**
   * TODO: Add description.
   */
  offset_y: 20;
}

export enum WindowPosition {
  TOP = 'top',
  BOTTOM = 'bottom',
  LEFT = 'left',
  RIGHT = 'right',
}
