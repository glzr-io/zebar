import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Switch } from '@/components/ui/switch';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { PlusCircle } from 'lucide-react';

type InstanceConfig = {
  anchor: string;
  offsetX: number;
  offsetY: number;
  monitor: string;
};

export default function WidgetSettings() {
  const [htmlPath, setHtmlPath] = useState('');
  const [zOrder, setZOrder] = useState('normal');
  const [shownInTaskbar, setShownInTaskbar] = useState(false);
  const [focused, setFocused] = useState(false);
  const [resizable, setResizable] = useState(true);
  const [transparent, setTransparent] = useState(false);
  const [windowEffect, setWindowEffect] = useState('none');
  const [backgroundColor, setBackgroundColor] = useState('#ffffff');
  const [defaultInstances, setDefaultInstances] = useState<
    InstanceConfig[]
  >([]);

  const addNewInstance = () => {
    setDefaultInstances([
      ...defaultInstances,
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
    const newInstances = [...defaultInstances];
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
          <Input
            id="html-path"
            value={htmlPath}
            onChange={e => setHtmlPath(e.target.value)}
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
              <Label htmlFor="z-order">Z-Order</Label>
              <Select value={zOrder} onValueChange={setZOrder}>
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
              </Select>
            </div>

            <div>
              <Label htmlFor="window-effect">Window Effect</Label>
              <Select value={windowEffect} onValueChange={setWindowEffect}>
                <SelectTrigger id="window-effect">
                  <SelectValue placeholder="Select Window Effect" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="mica">Mica</SelectItem>
                  <SelectItem value="acrylic">Acrylic</SelectItem>
                  <SelectItem value="none">None</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>

          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <Label htmlFor="shown-in-taskbar">Shown in taskbar</Label>
              <Switch
                id="shown-in-taskbar"
                checked={shownInTaskbar}
                onCheckedChange={setShownInTaskbar}
              />
            </div>

            <div class="flex items-center justify-between">
              <Label htmlFor="focused">Focused</Label>
              <Switch
                id="focused"
                checked={focused}
                onCheckedChange={setFocused}
              />
            </div>

            <div class="flex items-center justify-between">
              <Label htmlFor="resizable">Resizable</Label>
              <Switch
                id="resizable"
                checked={resizable}
                onCheckedChange={setResizable}
              />
            </div>

            <div class="flex items-center justify-between">
              <Label htmlFor="transparent">Transparent</Label>
              <Switch
                id="transparent"
                checked={transparent}
                onCheckedChange={setTransparent}
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
          <Label htmlFor="background-color">Background Color</Label>
          <div class="flex items-center space-x-2">
            <Input
              id="background-color"
              type="color"
              value={backgroundColor}
              onChange={e => setBackgroundColor(e.target.value)}
              class="w-12 h-12 p-1 rounded"
            />
            <Input
              value={backgroundColor}
              onChange={e => setBackgroundColor(e.target.value)}
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
          {defaultInstances.map((instance, index) => (
            <div class="border p-4 rounded-md space-y-2">
              <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                <div>
                  <Label htmlFor={`anchor-${index}`}>Anchor</Label>
                  <Select
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
                  </Select>
                </div>

                <div>
                  <Label htmlFor={`monitor-${index}`}>Monitor</Label>
                  <Select
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
                  </Select>
                </div>
              </div>

              <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                <div>
                  <Label htmlFor={`offset-x-${index}`}>Offset X</Label>
                  <Input
                    id={`offset-x-${index}`}
                    type="number"
                    value={instance.offsetX}
                    onChange={e =>
                      updateInstance(
                        index,
                        'offsetX',
                        parseInt(e.target.value),
                      )
                    }
                  />
                </div>

                <div>
                  <Label htmlFor={`offset-y-${index}`}>Offset Y</Label>
                  <Input
                    id={`offset-y-${index}`}
                    type="number"
                    value={instance.offsetY}
                    onChange={e =>
                      updateInstance(
                        index,
                        'offsetY',
                        parseInt(e.target.value),
                      )
                    }
                  />
                </div>
              </div>
            </div>
          ))}

          <Button onClick={addNewInstance} class="w-full">
            <PlusCircle class="mr-2 h-4 w-4" /> Add New Instance Config
          </Button>
        </CardContent>
      </Card>

      <Button class="w-full">Launch Default</Button>
    </div>
  );
}
