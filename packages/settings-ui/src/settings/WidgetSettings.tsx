import { invoke } from '@tauri-apps/api/core';
import { createResource } from 'solid-js';

import { WidgetConfigTree } from './WidgetConfigTree';
import { WidgetSettingsForm } from './WidgetSettingsForm';

export interface WidgetConfigEntry {
  /**
   * Absolute path to the widget's config file.
   */
  configPath: string;

  /**
   * Absolute path to the widget's HTML file.
   */
  htmlPath: string;

  /**
   * Parsed widget config.
   */
  config: WidgetConfig;
}

export type WidgetConfig = {
  htmlPath: string;
  zOrder: 'normal' | 'top_most' | 'bottom_most';
  shownInTaskbar: boolean;
  focused: boolean;
  resizable: boolean;
  transparent: boolean;
  backgroundColor: string;
  presets: WidgetPreset[];
};

export type WidgetPreset = {
  name: string;
  anchor:
    | 'top_left'
    | 'top_center'
    | 'top_right'
    | 'center'
    | 'bottom_left'
    | 'bottom_center'
    | 'bottom_right';
  offsetX: string;
  offsetY: string;
  width: string;
  height: string;
  monitorSelection: {
    type: 'all' | 'primary' | 'secondary';
  };
};

export function WidgetSettings() {
  const [widgetConfigs] = createResource(async () => {
    const xx = await invoke<any>('widget_configs');
    console.log('widgetConfigs', xx);
    return xx;
  });

  return (
    <div class="grid grid-cols-[minmax(200px,_min(25%,_400px))_1fr]">
      <WidgetConfigTree
        configs={widgetConfigs() ?? []}
        onSelect={() => {}}
      />
      <WidgetSettingsForm />
    </div>
  );
}
