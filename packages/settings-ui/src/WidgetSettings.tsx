import { createSignal } from 'solid-js';
import {
  Button,
  NumberInput,
  TextInput,
  FormLabel,
  SelectInput,
  SwitchInput,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from '@glzr/components';
// import { PlusCircle } from 'lucide-react';

type InstanceConfig = {
  anchor: string;
  offsetX: number;
  offsetY: number;
  monitor: string;
};

export function WidgetSettings() {
  const [htmlPath, setHtmlPath] = createSignal('');
  const [zOrder, setZOrder] = createSignal('normal');
  const [shownInTaskbar, setShownInTaskbar] = createSignal(false);
  const [focused, setFocused] = createSignal(false);
  const [resizable, setResizable] = createSignal(true);
  const [transparent, setTransparent] = createSignal(false);
  const [windowEffect, setWindowEffect] = createSignal('none');
  const [backgroundColor, setBackgroundColor] = createSignal('#ffffff');
  const [defaultInstances, setDefaultInstances] = createSignal<
    InstanceConfig[]
  >([]);

  const addNewInstance = () => {
    setDefaultInstances([
      ...defaultInstances(),
      {
        anchor: 'top-left',
        offsetX: 0,
        offsetY: 0,
        monitor: 'primary',
      },
    ]);
  };

  const updateInstance = (
    index: number,
    key: keyof InstanceConfig,
    value: string | number,
  ) => {
    const newInstances = [...defaultInstances()];
    newInstances[index] = { ...newInstances[index], [key]: value };
    setDefaultInstances(newInstances);
  };

  return (
    <div class="container mx-auto p-4 space-y-8">
      <h1 class="text-2xl font-bold">Widget Settings</h1>

      <Card>
        <CardHeader>
          <CardTitle>HTML Path</CardTitle>
        </CardHeader>
        <CardContent>
          <TextInput
            id="html-path"
            value={htmlPath()}
            onChange={setHtmlPath}
            placeholder="/path/to/widget.html"
          />
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Instance Options</CardTitle>
        </CardHeader>
        <CardContent class="space-y-4">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <FormLabel for="z-order">Z-Order</FormLabel>
              <SelectInput
                value={zOrder()}
                onChange={setZOrder}
                placeholder="Select Z-Order"
                options={[
                  {
                    value: 'normal',
                    label: 'Normal',
                  },
                  {
                    value: 'top-most',
                    label: 'Top-most (above all)',
                  },
                  {
                    value: 'bottom-most',
                    label: 'Bottom-most (on desktop)',
                  },
                ]}
              />
            </div>

            <div>
              <FormLabel for="window-effect">Window Effect</FormLabel>

              <SelectInput
                value={windowEffect()}
                onChange={setWindowEffect}
                placeholder="Select Window Effect"
                options={[
                  { value: 'mica', label: 'Mica' },
                  { value: 'acrylic', label: 'Acrylic' },
                  { value: 'none', label: 'None' },
                ]}
              />
            </div>
          </div>

          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <FormLabel for="shown-in-taskbar">
                Shown in taskbar
              </FormLabel>
              <SwitchInput
                id="shown-in-taskbar"
                value={shownInTaskbar()}
                onChange={setShownInTaskbar}
              />
            </div>

            <div class="flex items-center justify-between">
              <FormLabel for="focused">Focused</FormLabel>
              <SwitchInput
                id="focused"
                value={focused()}
                onChange={setFocused}
              />
            </div>

            <div class="flex items-center justify-between">
              <FormLabel for="resizable">Resizable</FormLabel>
              <SwitchInput
                id="resizable"
                value={resizable()}
                onChange={setResizable}
              />
            </div>

            <div class="flex items-center justify-between">
              <FormLabel for="transparent">Transparent</FormLabel>
              <SwitchInput
                id="transparent"
                value={transparent()}
                onChange={setTransparent}
              />
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Inputs</CardTitle>
        </CardHeader>
        <CardContent>
          <FormLabel for="background-color">Background Color</FormLabel>
          <div class="flex items-center space-x-2">
            <TextInput
              id="background-color"
              type="color"
              value={backgroundColor()}
              onChange={setBackgroundColor}
              class="w-12 h-12 p-1 rounded"
            />
            <TextInput
              value={backgroundColor()}
              onChange={setBackgroundColor}
              class="flex-grow"
            />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Default Instances</CardTitle>
        </CardHeader>
        <CardContent class="space-y-4">
          {defaultInstances().map((instance, index) => (
            <div class="border p-4 rounded-md space-y-2">
              <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                <div>
                  <FormLabel for={`anchor-${index}`}>Anchor</FormLabel>
                  <SelectInput
                    value={instance.anchor}
                    onChange={value =>
                      updateInstance(index, 'anchor', value)
                    }
                    options={[
                      { value: 'top-left', label: 'Top Left' },
                      { value: 'top-right', label: 'Top Right' },
                      { value: 'bottom-left', label: 'Bottom Left' },
                      { value: 'bottom-right', label: 'Bottom Right' },
                    ]}
                  />
                </div>

                <div>
                  <FormLabel for={`monitor-${index}`}>Monitor</FormLabel>
                  <SelectInput
                    value={instance.monitor}
                    onChange={value =>
                      updateInstance(index, 'monitor', value)
                    }
                    options={[
                      { value: 'primary', label: 'Primary' },
                      { value: 'secondary', label: 'Secondary' },
                      { value: 'all', label: 'All' },
                    ]}
                  />
                </div>
              </div>

              <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                <div>
                  <FormLabel for={`offset-x-${index}`}>Offset X</FormLabel>
                  <NumberInput
                    id={`offset-x-${index}`}
                    value={instance.offsetX}
                    onChange={e => updateInstance(index, 'offsetX', e)}
                  />
                </div>

                <div>
                  <FormLabel for={`offset-y-${index}`}>Offset Y</FormLabel>
                  <NumberInput
                    id={`offset-y-${index}`}
                    value={instance.offsetY}
                    onChange={e => updateInstance(index, 'offsetY', e)}
                  />
                </div>
              </div>
            </div>
          ))}

          <Button onClick={addNewInstance} class="w-full">
            {/* <PlusCircle class="mr-2 h-4 w-4" /> Add New Instance Config */}
          </Button>
        </CardContent>
      </Card>

      <Button class="w-full">Launch Default</Button>
    </div>
  );
}
