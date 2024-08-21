export interface WindowConfig {
  /**
   * Whether to show the window above/below all others.
   */
  z_order: WindowZOrder;

  /**
   * Whether the window should be shown in the taskbar.
   */
  shown_in_taskbar: boolean;

  /**
   * Whether the window should have resize handles.
   */
  resizable: boolean;

  /**
   * Whether the window is transparent.
   */
  transparent: boolean;

  /**
   * Entry point HTML file for the window.
   */
  html_path: string;

  /**
   * Where to place the window.
   */
  placements: WindowPlacement[];
}

export enum WindowZOrder {
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
