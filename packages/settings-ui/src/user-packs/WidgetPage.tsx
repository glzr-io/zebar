import {
  Button,
  cn,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@glzr/components';
import { useParams } from '@solidjs/router';
import { IconChevronDown } from '@tabler/icons-solidjs';
import {
  createEffect,
  createMemo,
  createSignal,
  For,
  on,
  Show,
} from 'solid-js';

import { WidgetConfigForm } from './WidgetConfigForm';
import { useUserPacks } from '~/common';

export function WidgetPage() {
  const params = useParams();

  const { allPacks, widgetStates, updateWidgetConfig, togglePreset } =
    useUserPacks();

  const selectedPack = createMemo(() =>
    allPacks().find(pack => pack.id === params.packId),
  );

  const selectedWidgetName = createMemo(() => params.widgetName);
  const selectedConfig = createMemo(() =>
    selectedPack()?.widgetConfigs.find(
      widget => widget.name === params.widgetName,
    ),
  );

  const [selectedPreset, setSelectedPreset] = createSignal<string | null>(
    null,
  );

  const presetNames = createMemo(() =>
    (selectedConfig()?.presets ?? []).map(preset => preset.name),
  );

  // Widget states for the selected config.
  const selectedConfigStates = createMemo(() => {
    const configPath = selectedWidgetName();
    return Object.values(widgetStates()).filter(
      state => state.configPath === configPath,
    );
  });

  // Widget states for the selected preset.
  const selectedPresetStates = createMemo(() => {
    const preset = selectedPreset();
    return selectedConfigStates().filter(
      // @ts-ignore - TODO
      state => state.openOptions?.preset === preset,
    );
  });

  // Initialize the selected preset when a config is selected.
  createEffect(
    on(
      () => selectedWidgetName(),
      () => {
        if (selectedConfig()) {
          setSelectedPreset(selectedConfig().presets[0]?.name ?? null);
          document.querySelector('#form-container').scrollTo(0, 0);
        }
      },
    ),
  );

  // Ensure selected preset is valid when presets change.
  createEffect(
    on(
      () => presetNames(),
      () => {
        if (
          !selectedPreset() ||
          !presetNames().includes(selectedPreset())
        ) {
          setSelectedPreset(presetNames()[0] ?? null);
        }
      },
    ),
  );

  return (
    <div class="flex h-screen bg-background">
      <Show when={selectedConfig()}>
        {config => (
          <main class="flex-1 grid grid-rows-[1fr_auto] overflow-hidden">
            <div id="form-container" class="container p-4 overflow-y-auto">
              <h1 class="text-2xl font-bold mb-1">
                {selectedWidgetName()}
              </h1>

              <p class="bg-muted text-xs font-mono rounded-sm mb-6 p-1 text-muted-foreground inline-block">
                {selectedWidgetName()}
              </p>

              <WidgetConfigForm
                config={config()}
                configPath={selectedWidgetName()}
                onChange={config =>
                  updateWidgetConfig(selectedWidgetName(), config)
                }
              />
            </div>

            {/* Action bar. */}
            <div class="flex items-center justify-end border-t p-4">
              <div class="flex items-center">
                <span class="text-sm font-normal text-muted-foreground mr-2">
                  {selectedConfigStates().length} open (
                  {selectedPresetStates().length} for preset)
                </span>

                <Button
                  class="rounded-r-none self-end"
                  disabled={presetNames().length === 0}
                  onClick={() =>
                    togglePreset(selectedWidgetName(), selectedPreset())
                  }
                >
                  <Show when={selectedPreset()} fallback="No presets">
                    {selectedPresetStates().length === 0
                      ? `Open ${selectedPreset()}`
                      : `Close ${selectedPreset()}`}
                  </Show>
                </Button>

                <DropdownMenu>
                  <DropdownMenuTrigger>
                    <Button
                      class="rounded-l-none border-l-0 px-2"
                      disabled={presetNames().length === 0}
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
