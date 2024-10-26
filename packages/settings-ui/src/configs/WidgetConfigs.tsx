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
import { listen, type Event } from '@tauri-apps/api/event';
import {
  createEffect,
  createMemo,
  createResource,
  createSignal,
  For,
  on,
  Show,
} from 'solid-js';
import { WidgetConfig } from 'zebar';

import { WidgetConfigSidebar } from './WidgetConfigSidebar';
import { WidgetConfigForm } from './WidgetConfigForm';

export function WidgetConfigs() {
  const [configs, { mutate: mutateWidgetConfigs }] = createResource(
    async () => invoke<Record<string, WidgetConfig>>('widget_configs'),
    { initialValue: {} },
  );

  const [widgetStates, { mutate: mutateWidgetStates }] = createResource(
    async () => invoke<Record<string, WidgetConfig>>('widget_states'),
    { initialValue: {} },
  );

  createEffect(() => console.log('widgetStates', widgetStates()));

  const [selectedConfigPath, setSelectedConfigPath] = createSignal<
    string | null
  >(null);

  const [selectedPreset, setSelectedPreset] = createSignal<string | null>(
    null,
  );

  const selectedConfig = createMemo(() => configs()[selectedConfigPath()]);

  const presetNames = createMemo(() =>
    (selectedConfig()?.presets ?? []).map(preset => preset.name),
  );

  // TODO: Get whether the selected preset is currently active.
  const isSelectedPresetOpen = createMemo(() => false);

  // Listen for changes to widget states.
  listen('widget-opened', (event: Event<any>) => {
    mutateWidgetStates(states => ({
      ...states,
      [event.payload.id]: event.payload,
    }));
  });

  // Listen for changes to widget states.
  listen('widget-closed', (event: Event<any>) => {
    mutateWidgetStates(states => {
      const newStates = { ...states };
      delete newStates[event.payload.id];
      return newStates;
    });
  });

  // Initialize the selected preset when a config is selected.
  createEffect(
    on(
      () => selectedConfigPath(),
      () => {
        if (selectedConfig()) {
          setSelectedPreset(selectedConfig().presets[0]?.name);
          document.querySelector('#form-container').scrollTo(0, 0);
        }
      },
    ),
  );

  async function onConfigChange(
    configPath: string,
    newConfig: WidgetConfig,
  ) {
    // Update the state with the new config values.
    mutateWidgetConfigs(configs => ({
      ...configs,
      [configPath]: newConfig,
    }));

    // Send updated config values to backend.
    await invoke<void>('update_widget_config', {
      configPath,
      newConfig,
    });
  }

  // TODO: Stop preset if `isSelectedPresetOpen` is `true`.
  async function togglePreset(configPath: string, presetName: string) {
    await invoke<void>('start_preset', {
      configPath,
      presetName,
    });
  }

  return (
    <div class="flex h-screen bg-background">
      {/* Sidebar. */}
      <WidgetConfigSidebar
        configs={configs()}
        selectedConfig={selectedConfig()}
        selectedConfigPath={selectedConfigPath()}
        onSelect={setSelectedConfigPath}
      />

      {/* Main content. */}
      <Show
        when={selectedConfig()}
        fallback={<WidgetSettingsEmptyState />}
      >
        {config => (
          <main class="flex-1 grid grid-rows-[1fr_auto] overflow-hidden">
            <div id="form-container" class="container p-4 overflow-y-auto">
              <h1 class="text-2xl font-bold mb-1">
                {selectedConfigPath().split(/[/\\]/).at(-1)}
              </h1>

              <p class="bg-muted text-xs font-mono rounded-sm mb-6 p-1 text-muted-foreground inline-block">
                {selectedConfigPath()}
              </p>

              <WidgetConfigForm
                config={config()}
                onChange={config =>
                  onConfigChange(selectedConfigPath(), config)
                }
              />
            </div>

            {/* Action bar. */}
            <div class="flex items-center justify-end border-t p-4">
              <div class="flex items-center">
                <Button
                  class="rounded-r-none self-end"
                  disabled={presetNames().length === 0}
                  onClick={() =>
                    togglePreset(selectedConfigPath(), selectedPreset())
                  }
                >
                  <Show when={selectedPreset()} fallback="No presets">
                    {isSelectedPresetOpen()
                      ? `Open ${selectedPreset()}`
                      : `Close ${selectedPreset()}`}
                  </Show>
                </Button>

                <DropdownMenu>
                  <DropdownMenuTrigger>
                    <Button class="rounded-l-none border-l-0 px-2">
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
