import { createSignal } from 'solid-js';
import { Button, NumberInput } from '@glzr/components';
import { TextInput } from '@glzr/components';
import { FormLabel } from '@glzr/components';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@glzr/components';
import { Switch } from '@glzr/components';
import {
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
              {/* <Select value={zOrder} onChange={setZOrder}>
                <SelectTrigger id="z-order">
                  <SelectValue placeholder="Select Z-Order" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="normal">Normal</SelectItem>
                  <SelectItem value="top-most">
                    Top-most (above all)
                  </SelectItem>
                  <SelectItem value="bottom-most">
                    Bottom-most (on desktop)
                  </SelectItem>
                </SelectContent>
              </Select> */}
            </div>

            <div>
              <FormLabel for="window-effect">Window Effect</FormLabel>
              {/* <Select value={windowEffect} onValueChange={setWindowEffect}>
                <SelectTrigger id="window-effect">
                  <SelectValue placeholder="Select Window Effect" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="mica">Mica</SelectItem>
                  <SelectItem value="acrylic">Acrylic</SelectItem>
                  <SelectItem value="none">None</SelectItem>
                </SelectContent>
              </Select> */}
            </div>
          </div>

          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <FormLabel for="shown-in-taskbar">
                Shown in taskbar
              </FormLabel>
              {/* <Switch
                id="shown-in-taskbar"
                checked={shownInTaskbar}
                onCheckedChange={setShownInTaskbar}
              /> */}
            </div>

            <div class="flex items-center justify-between">
              <FormLabel for="focused">Focused</FormLabel>
              {/* <Switch
                id="focused"
                checked={focused}
                onCheckedChange={setFocused}
              /> */}
            </div>

            <div class="flex items-center justify-between">
              <FormLabel for="resizable">Resizable</FormLabel>
              {/* <Switch
                id="resizable"
                checked={resizable}
                onCheckedChange={setResizable}
              /> */}
            </div>

            <div class="flex items-center justify-between">
              <FormLabel for="transparent">Transparent</FormLabel>
              {/* <Switch
                id="transparent"
                checked={transparent}
                onCheckedChange={setTransparent}
              /> */}
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
                  {/* <Select
                    value={instance.anchor}
                    onValueChange={value =>
                      updateInstance(index, 'anchor', value)
                    }
                  >
                    <SelectTrigger id={`anchor-${index}`}>
                      <SelectValue placeholder="Select Anchor" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="top-left">Top Left</SelectItem>
                      <SelectItem value="top-right">Top Right</SelectItem>
                      <SelectItem value="bottom-left">
                        Bottom Left
                      </SelectItem>
                      <SelectItem value="bottom-right">
                        Bottom Right
                      </SelectItem>
                    </SelectContent>
                  </Select> */}
                </div>

                <div>
                  <FormLabel for={`monitor-${index}`}>Monitor</FormLabel>
                  {/* <Select
                    value={instance.monitor}
                    onValueChange={value =>
                      updateInstance(index, 'monitor', value)
                    }
                  >
                    <SelectTrigger id={`monitor-${index}`}>
                      <SelectValue placeholder="Select Monitor" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="primary">Primary</SelectItem>
                      <SelectItem value="secondary">Secondary</SelectItem>
                      <SelectItem value="all">All</SelectItem>
                    </SelectContent>
                  </Select> */}
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
