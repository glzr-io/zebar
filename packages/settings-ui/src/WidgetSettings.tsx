import {
  Button,
  FormLabel,
  SelectInput,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  TextField,
  SelectField,
  SwitchField,
} from '@glzr/components';
import { createForm, Field } from 'smorf';

type WidgetSettings = {
  htmlPath: string;
  zOrder: 'normal' | 'top_most' | 'bottom_most';
  shownInTaskbar: boolean;
  focused: boolean;
  resizable: boolean;
  transparent: boolean;
  backgroundColor: string;
  presets: WidgetPreset[];
};

type WidgetPreset = {
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
  const widgetForm = createForm<WidgetSettings>({
    htmlPath: '',
    zOrder: 'normal',
    shownInTaskbar: false,
    focused: false,
    resizable: true,
    transparent: false,
    backgroundColor: '#ffffff',
    presets: [],
  });

  function addNewInstance() {
    widgetForm.setValue('presets', presets => [
      ...presets,
      {
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
          <CardTitle>Widget Settings</CardTitle>
        </CardHeader>
        <CardContent class="space-y-4">
          <Field of={widgetForm} path="htmlPath">
            {inputProps => (
              <TextField
                id="html-path"
                label="HTML Path"
                placeholder="/path/to/widget.html"
                {...inputProps()}
              />
            )}
          </Field>

          <Field of={widgetForm} path="zOrder">
            {inputProps => (
              <SelectField
                label="Z-Order"
                placeholder="Select Z-Order"
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

          <Field of={widgetForm} path="shownInTaskbar">
            {inputProps => (
              <SwitchField
                id="shown-in-taskbar"
                label="Shown in taskbar"
                {...inputProps()}
              />
            )}
          </Field>

          <Field of={widgetForm} path="focused">
            {inputProps => (
              <SwitchField
                id="focused"
                label="Focused"
                {...inputProps()}
              />
            )}
          </Field>

          <Field of={widgetForm} path="resizable">
            {inputProps => (
              <SwitchField
                id="resizable"
                label="Resizable"
                {...inputProps()}
              />
            )}
          </Field>

          <Field of={widgetForm} path="transparent">
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
          {widgetForm.value.presets.map((instance, index) => (
            <div class="border p-4 rounded-md space-y-2">
              <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                <div>
                  <FormLabel for={`anchor-${index}`}>Anchor</FormLabel>
                  <SelectInput
                    value={instance.anchor}
                    onChange={value =>
                      widgetForm.setValue(`presets.${index}`, preset => ({
                        ...preset,
                        anchor: value,
                      }))
                    }
                    options={
                      [
                        { value: 'top_left', label: 'Top Left' },
                        { value: 'top_right', label: 'Top Right' },
                        { value: 'bottom_left', label: 'Bottom Left' },
                        { value: 'bottom_right', label: 'Bottom Right' },
                      ] as const
                    }
                  />
                </div>

                <div>
                  <FormLabel for={`monitor-${index}`}>Monitor</FormLabel>
                  <SelectInput
                    value={instance.monitorSelection.type}
                    onChange={value =>
                      widgetForm.setValue(`presets.${index}`, preset => ({
                        ...preset,
                        monitor: value,
                      }))
                    }
                    options={
                      [
                        { value: 'primary', label: 'Primary' },
                        { value: 'secondary', label: 'Secondary' },
                        { value: 'all', label: 'All' },
                      ] as const
                    }
                  />
                </div>
              </div>

              <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                {/* TODO: Change to px/percent input. */}
                <TextField
                  id={`offset-x-${index}`}
                  label="Offset X"
                  value={instance.offsetX}
                  onChange={offsetX =>
                    widgetForm.setValue(`presets.${index}`, preset => ({
                      ...preset,
                      offsetX,
                    }))
                  }
                />

                {/* TODO: Change to px/percent input. */}
                <TextField
                  id={`offset-y-${index}`}
                  label="Offset Y"
                  value={instance.offsetY}
                  onChange={offsetY =>
                    widgetForm.setValue(`presets.${index}`, preset => ({
                      ...preset,
                      offsetY,
                    }))
                  }
                />
              </div>
            </div>
          ))}

          <Button onClick={addNewInstance} class="w-full">
            Add new instance config +
            {/* <PlusCircle class="mr-2 h-4 w-4" />  */}
          </Button>
        </CardContent>
      </Card>

      <Button class="w-full">Launch Default</Button>
    </div>
  );
}
