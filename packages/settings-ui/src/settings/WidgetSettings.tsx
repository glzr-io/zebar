import {
  Button,
  cn,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  IconChevronDown,
} from '@glzr/components';
import { invoke } from '@tauri-apps/api/core';
import {
  createMemo,
  createResource,
  createSignal,
  For,
  Show,
} from 'solid-js';

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
    const xx = await invoke<WidgetConfigEntry[]>('widget_configs');
    console.log('widgetConfigs', xx);
    return xx;
  });

  const [selectedConfigPath, setSelectedConfigPath] = createSignal<
    string | null
  >(null);

  const [selectedPreset, setSelectedPreset] = createSignal<string | null>(
    null,
  );

  const presetNames = createMemo(() =>
    (widgetConfigs() ?? [])
      .flatMap(entry => entry.config.presets)
      .map(preset => preset.name),
  );

  const selectedConfig = createMemo(
    () =>
      (widgetConfigs() ?? []).find(
        entry => entry.configPath === selectedConfigPath(),
      )?.config,
  );

  function onConfigChange(config: WidgetConfig) {
    // TODO: Send the updated config to the backend.
    // TODO: Update the `widgetConfigs` resource.
  }

  function openWidgetWithPreset() {
    // TODO: Implement this.
  }

  return (
    <div class="grid grid-cols-[minmax(200px,_min(25%,_400px))_1fr]">
      <WidgetConfigTree
        configs={widgetConfigs() ?? []}
        onSelect={setSelectedConfigPath}
      />

      <div>
        <Show when={selectedConfig()}>
          <WidgetSettingsForm
            config={selectedConfig()}
            onChange={onConfigChange}
          />
        </Show>

        <div class="flex items-center">
          <Button
            variant="outline"
            class="rounded-r-none"
            onClick={() => openWidgetWithPreset()}
          >
            {selectedPreset() ?? 'Select'}
          </Button>
          <DropdownMenu>
            <DropdownMenuTrigger>
              <Button
                variant="outline"
                class="rounded-l-none border-l-0 px-2"
              >
                <IconChevronDown class="size-3" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent>
              <For each={presetNames()}>
                {presetName => (
                  <DropdownMenuItem
                    onClick={() => setSelectedPreset(presetName)}
                    class={cn({
                      'bg-accent text-accent-foreground':
                        presetName === selectedPreset(),
                    })}
                  >
                    {presetName}
                  </DropdownMenuItem>
                )}
              </For>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </div>
    </div>
  );
}
