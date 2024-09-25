export interface WidgetConfig {
  /**
   * Entry point HTML file for the window.
   */
  htmlPath: string;

  /**
   * Whether to show the window above/below all others.
   */
  zOrder: ZOrder;

  /**
   * Whether the window should be focused when opened.
   */
  focused: boolean;

  /**
   * Whether the window should be shown in the taskbar.
   */
  shownInTaskbar: boolean;

  /**
   * Whether the window should have resize handles.
   */
  resizable: boolean;

  /**
   * Whether the window frame should be transparent.
   */
  transparent: boolean;

  /**
   * Where to place the window.
   */
  defaultPlacements: WidgetPlacement[];
}

export type ZOrder = 'always_on_bottom' | 'always_on_top' | 'normal';

export interface WidgetPlacement {
  /**
   * Anchor-point of the window.
   */
  anchor: WindowAnchor;

  /**
   * Offset from the anchor-point.
   */
  offsetX: LengthValue;

  /**
   * Offset from the anchor-point.
   */
  offsetY: LengthValue;

  /**
   * Width of the window in % or physical pixels.
   */
  width: LengthValue;

  /**
   * Height of the window in % or physical pixels.
   */
  height: LengthValue;

  /**
   * Monitor(s) to place the window on.
   */
  monitor_selection: MonitorSelection;
}

export type PlacementAnchor =
  | 'top_left'
  | 'top_center'
  | 'top_right'
  | 'center_left'
  | 'center'
  | 'center_right'
  | 'bottom_left'
  | 'bottom_center'
  | 'bottom_right';
