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
  createEffect,
  createMemo,
  createResource,
  createSignal,
  For,
  on,
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
  const [configEntries, { mutate }] = createResource(async () => {
    const xx = await invoke<WidgetConfigEntry[]>('widget_configs');
    console.log('widget_configs', xx);
    return xx;
  });

  const [selectedConfigPath, setSelectedConfigPath] = createSignal<
    string | null
  >(null);

  const [selectedPreset, setSelectedPreset] = createSignal<string | null>(
    null,
  );

  const selectedConfigEntry = createMemo(() => {
    const configPath = selectedConfigPath();
    return (configEntries() ?? []).find(
      entry => entry.configPath === configPath,
    );
  });

  const presetNames = createMemo(() =>
    (selectedConfigEntry()?.config.presets ?? []).map(
      preset => preset.name,
    ),
  );

  // Initialize the selected preset when a config is selected.
  createEffect(
    on(
      () => selectedConfigPath(),
      () => {
        if (selectedConfigEntry()) {
          setSelectedPreset(selectedConfigEntry().config.presets[0]?.name);
        }
      },
    ),
  );

  async function onConfigChange(newConfig: WidgetConfig) {
    // Update the state with the new config values.
    mutate(entries =>
      entries.map(entry =>
        entry.configPath !== selectedConfigPath()
          ? entry
          : { ...entry, config: newConfig },
      ),
    );

    // Send updated config values to backend.
    await invoke<void>('update_widget_config', {
      configPath: selectedConfigPath(),
      newConfig,
    });
  }

  function openWidgetWithPreset() {
    // TODO: Implement this.
  }

  return (
    <div class="flex h-screen bg-background">
      {/* Sidebar. */}
      <WidgetConfigTree
        configEntries={configEntries() ?? []}
        selectedEntry={selectedConfigEntry()}
        onSelect={setSelectedConfigPath}
      />

      {/* Main content area. */}
      <Show
        when={selectedConfigEntry()}
        fallback={<WidgetSettingsEmptyState />}
      >
        {configEntry => (
          <main class="flex-1 grid grid-rows-[1fr_auto] overflow-hidden">
            <div class="container p-4 overflow-y-auto">
              <h1 class="text-2xl font-bold mb-1">
                {configEntry().configPath.split(/[/\\]/).at(-1)}
              </h1>

              <p class="bg-muted text-xs font-mono rounded-sm mb-6 p-1 text-muted-foreground inline-block">
                {configEntry().configPath}
              </p>

              <WidgetSettingsForm
                config={configEntry().config}
                onChange={onConfigChange}
              />
            </div>

            {/* Action bar. */}
            <div class="flex items-center justify-end border-t p-4">
              <div class="flex items-center">
                <Button
                  variant="outline"
                  class="rounded-r-none self-end"
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
          </main>
        )}
      </Show>
    </div>
  );
}

function WidgetSettingsEmptyState() {
  return <p class="p-4">No config selected.</p>;
}
