import {
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  TextField,
  SelectField,
  SwitchField,
  IconTrash,
  TooltipContent,
  Tooltip,
  TooltipTrigger,
} from '@glzr/components';
import { createForm, Field } from 'smorf';
import { createEffect, on } from 'solid-js';
import { WidgetConfig } from 'zebar';

export interface WidgetConfigFormProps {
  config: WidgetConfig;
  onChange: (config: WidgetConfig) => void;
}

export function WidgetConfigForm(props: WidgetConfigFormProps) {
  const configForm = createForm<WidgetConfig>(props.config);

  // Update the form when the incoming config changes.
  createEffect(
    on(
      () => props.config,
      config => configForm.setValue(config),
    ),
  );

  // Emit changes to the form value.
  createEffect(
    on(
      () => configForm.value,
      formValue => {
        if (configForm.isDirty()) {
          props.onChange(formValue);
        }
      },
    ),
  );

  function addNewPreset() {
    configForm.setFieldValue('presets', presets => [
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

  function deletePreset(targetIndex: number) {
    configForm.setFieldValue('presets', presets =>
      presets.filter((_, index) => index !== targetIndex),
    );
  }

  return (
    <div class="space-y-8">
      <Card>
        <CardHeader>
          <CardTitle>Widget settings</CardTitle>
        </CardHeader>

        <CardContent class="space-y-4">
          <Field of={configForm} path="htmlPath">
            {inputProps => (
              <TextField
                id="html-path"
                label="HTML path"
                placeholder="/path/to/widget.html"
                {...inputProps()}
              />
            )}
          </Field>

          <Field of={configForm} path="zOrder">
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

          <Field of={configForm} path="shownInTaskbar">
            {inputProps => (
              <SwitchField
                id="shown-in-taskbar"
                label="Shown in taskbar"
                {...inputProps()}
              />
            )}
          </Field>

          <Field of={configForm} path="focused">
            {inputProps => (
              <SwitchField
                id="focused"
                label="Focused on launch"
                {...inputProps()}
              />
            )}
          </Field>

          <Field of={configForm} path="resizable">
            {inputProps => (
              <SwitchField
                id="resizable"
                label="Resizable"
                {...inputProps()}
              />
            )}
          </Field>

          <Field of={configForm} path="transparent">
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
          {configForm.value.presets.map((_, index) => (
            <div class="border p-4 rounded-md space-y-2">
              <div class="flex justify-between">
                <Field of={configForm} path={`presets.${index}.name`}>
                  {inputProps => (
                    <TextField
                      id={`name-${index}`}
                      label="Preset name"
                      {...inputProps()}
                    />
                  )}
                </Field>

                <Tooltip openDelay={0} closeDelay={0}>
                  <TooltipTrigger
                    as={(props: any) => (
                      <Button
                        {...props}
                        variant="secondary"
                        size="icon"
                        onClick={() => deletePreset(index)}
                      >
                        <IconTrash class="size-4" />
                      </Button>
                    )}
                  />
                  <TooltipContent>Delete preset</TooltipContent>
                </Tooltip>
              </div>

              <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                <Field of={configForm} path={`presets.${index}.anchor`}>
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
                  of={configForm}
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
                <Field of={configForm} path={`presets.${index}.offsetX`}>
                  {inputProps => (
                    <TextField
                      id={`offset-x-${index}`}
                      label="Offset X"
                      {...inputProps()}
                    />
                  )}
                </Field>

                {/* TODO: Change to px/percent input. */}
                <Field of={configForm} path={`presets.${index}.offsetY`}>
                  {inputProps => (
                    <TextField
                      id={`offset-y-${index}`}
                      label="Offset Y"
                      {...inputProps()}
                    />
                  )}
                </Field>
              </div>

              <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                {/* TODO: Change to px/percent input. */}
                <Field of={configForm} path={`presets.${index}.width`}>
                  {inputProps => (
                    <TextField
                      id={`width-${index}`}
                      label="Width"
                      {...inputProps()}
                    />
                  )}
                </Field>

                {/* TODO: Change to px/percent input. */}
                <Field of={configForm} path={`presets.${index}.height`}>
                  {inputProps => (
                    <TextField
                      id={`height-${index}`}
                      label="Height"
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
