import {
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  TextField,
  SelectField,
  SwitchField,
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
  cn,
} from '@glzr/components';
import { createForm, Field } from 'smorf';
import { createMemo, createSignal, For } from 'solid-js';
import { WidgetConfig } from './WidgetSettings';

export function WidgetSettingsForm() {
  const settingsForm = createForm<WidgetConfig>({
    htmlPath: '',
    zOrder: 'normal',
    shownInTaskbar: false,
    focused: false,
    resizable: true,
    transparent: false,
    backgroundColor: '#ffffff',
    presets: [],
  });

  const presetNames = createMemo(() =>
    settingsForm.value.presets.map(preset => preset.name),
  );

  const [selectedPreset, setSelectedPreset] = createSignal<string | null>(
    null,
  );

  function addNewPreset() {
    settingsForm.setValue('presets', presets => [
      ...presets,
      {
        name: presets.length ? `default${presets.length + 1}` : 'default',
        anchor: 'center',
        offsetX: '0px',
        offsetY: '0px',
        width: '50%',
        height: '50%',
        monitorSelection: {
          type: 'all',
        },
      },
    ]);

    if (!selectedPreset()) {
      setSelectedPreset(settingsForm.value.presets[0].name);
    }
  }

  function openWidgetWithPreset() {
    // TODO: Implement this.
  }

  return (
    <div class="container mx-auto p-4 space-y-8">
      <h1 class="text-2xl font-bold">Widget / %NAME%</h1>

      <Card>
        <CardHeader>
          <CardTitle>Widget settings</CardTitle>
        </CardHeader>

        <CardContent class="space-y-4">
          <Field of={settingsForm} path="htmlPath">
            {inputProps => (
              <TextField
                id="html-path"
                label="HTML path"
                placeholder="/path/to/widget.html"
                {...inputProps()}
              />
            )}
          </Field>

          <Field of={settingsForm} path="zOrder">
            {inputProps => (
              <SelectField
                id="z-order"
                label="Z-order"
                placeholder="Select z-order"
                options={[
                  {
                    value: 'normal',
                    label: 'Normal',
                  },
                  {
                    value: 'top_most',
                    label: 'Top-most (above all)',
                  },
                  {
                    value: 'bottom_most',
                    label: 'Bottom-most (on desktop)',
                  },
                ]}
                {...inputProps()}
              />
            )}
          </Field>

          <Field of={settingsForm} path="shownInTaskbar">
            {inputProps => (
              <SwitchField
                id="shown-in-taskbar"
                label="Shown in taskbar"
                {...inputProps()}
              />
            )}
          </Field>

          <Field of={settingsForm} path="focused">
            {inputProps => (
              <SwitchField
                id="focused"
                label="Focused"
                {...inputProps()}
              />
            )}
          </Field>

          <Field of={settingsForm} path="resizable">
            {inputProps => (
              <SwitchField
                id="resizable"
                label="Resizable"
                {...inputProps()}
              />
            )}
          </Field>

          <Field of={settingsForm} path="transparent">
            {inputProps => (
              <SwitchField
                id="transparent"
                label="Transparent"
                {...inputProps()}
              />
            )}
          </Field>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Presets</CardTitle>
        </CardHeader>

        <CardContent class="space-y-4">
          {settingsForm.value.presets.map((_, index) => (
            <div class="border p-4 rounded-md space-y-2">
              <Field of={settingsForm} path={`presets.${index}.name`}>
                {inputProps => (
                  <TextField
                    id={`name-${index}`}
                    label="Preset name"
                    {...inputProps()}
                  />
                )}
              </Field>

              <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                <Field of={settingsForm} path={`presets.${index}.anchor`}>
                  {inputProps => (
                    <SelectField
                      id={`anchor-${index}`}
                      label="Anchor"
                      options={
                        [
                          { value: 'top_left', label: 'Top left' },
                          { value: 'top_center', label: 'Top center' },
                          { value: 'top_right', label: 'Top right' },
                          { value: 'center', label: 'Center' },
                          { value: 'bottom_left', label: 'Bottom left' },
                          {
                            value: 'bottom_center',
                            label: 'Bottom center',
                          },
                          { value: 'bottom_right', label: 'Bottom right' },
                        ] as const
                      }
                      {...inputProps()}
                    />
                  )}
                </Field>

                <Field
                  of={settingsForm}
                  path={`presets.${index}.monitorSelection.type`}
                >
                  {inputProps => (
                    <SelectField
                      id={`monitor-${index}`}
                      label="Target monitor(s)"
                      options={
                        [
                          { value: 'primary', label: 'Primary' },
                          { value: 'secondary', label: 'Secondary' },
                          { value: 'all', label: 'All' },
                        ] as const
                      }
                      {...inputProps()}
                    />
                  )}
                </Field>
              </div>

              <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                {/* TODO: Change to px/percent input. */}
                <Field of={settingsForm} path={`presets.${index}.offsetX`}>
                  {inputProps => (
                    <TextField
                      id={`offset-x-${index}`}
                      label="Offset X"
                      {...inputProps()}
                    />
                  )}
                </Field>

                {/* TODO: Change to px/percent input. */}
                <Field of={settingsForm} path={`presets.${index}.offsetY`}>
                  {inputProps => (
                    <TextField
                      id={`offset-y-${index}`}
                      label="Offset Y"
                      {...inputProps()}
                    />
                  )}
                </Field>
              </div>
            </div>
          ))}

          <Button onClick={addNewPreset} class="w-full">
            Add new preset +
          </Button>
        </CardContent>
      </Card>

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
              <svg
                class="size-3"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
              >
                <path
                  fill="none"
                  stroke="currentColor"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="m6 9l6 6l6-6"
                />
              </svg>
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
  );
}
