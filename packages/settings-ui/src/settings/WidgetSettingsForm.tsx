import {
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  TextField,
  SelectField,
  SwitchField,
} from '@glzr/components';
import { createForm, Field } from 'smorf';

import { WidgetConfig } from './WidgetSettings';
import { createEffect } from 'solid-js';

export interface WidgetSettingsFormProps {
  config: WidgetConfig;
  onChange: (config: WidgetConfig) => void;
}

export function WidgetSettingsForm(props: WidgetSettingsFormProps) {
  const settingsForm = createForm<WidgetConfig>(props.config);

  createEffect(() => {
    console.log('change', settingsForm.value);
    props.onChange(settingsForm.value);
  });

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
    </div>
  );
}
