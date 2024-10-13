import { invoke } from '@tauri-apps/api/core';
import { createResource } from 'solid-js';

import { FileItem, WidgetConfigTree } from './WidgetConfigTree';
import { WidgetSettingsForm } from './WidgetSettingsForm';

const initialFiles: FileItem[] = [
  {
    name: 'Documents',
    type: 'folder',
    children: [
      {
        name: 'Project Proposal.docx',
        type: 'file',
        size: '2.3 MB',
        modified: '2023-10-05',
      },
      {
        name: 'Budget.xlsx',
        type: 'file',
        size: '1.8 MB',
        modified: '2023-10-06',
      },
    ],
  },
  {
    name: 'Images',
    type: 'folder',
    children: [
      {
        name: 'Vacation.jpg',
        type: 'file',
        size: '3.2 MB',
        modified: '2023-09-28',
      },
      {
        name: 'Family.png',
        type: 'file',
        size: '2.9 MB',
        modified: '2023-09-29',
      },
    ],
  },
  {
    name: 'Music.mp3',
    type: 'file',
    size: '5.4 MB',
    modified: '2023-10-01',
  },
  {
    name: 'Video.mp4',
    type: 'file',
    size: '15.7 MB',
    modified: '2023-10-02',
  },
];

interface WidgetConfigEntry {
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
  const widgetConfigs = createResource(async () => {
    const xx = await invoke<any>('widget_configs');
    console.log('widgetConfigs', xx);
    return xx;
  });

  return (
    <div class="grid grid-cols-[minmax(200px,_min(25%,_400px))_1fr]">
      <WidgetConfigTree files={initialFiles} onSelect={() => {}} />
      <WidgetSettingsForm />
    </div>
  );
}
