import { Button, SelectInput } from '@glzr/components';
import { IconPlayerPause } from '@tabler/icons-solidjs';
import { createMemo } from 'solid-js';
import type { WidgetPack } from 'zebar';

export type PreviewBarProps = {
  pack: WidgetPack;
  widgetName: string;
  presetName: string;
  onChange: (widgetName: string, presetName: string) => void;
  onStop: () => void;
};

export function PreviewBar(props: PreviewBarProps) {
  const widgetOptions = createMemo(() =>
    props.pack.widgets.map(widget => ({
      value: widget.name,
      label: widget.name,
    })),
  );

  const presetOptions = createMemo(
    () =>
      props.pack.widgets
        .find(widget => widget.name === props.widgetName)
        ?.presets.map(preset => ({
          value: preset.name,
          label: preset.name,
        })) ?? [],
  );

  return (
    <div class="fixed bottom-4 left-1/2 -translate-x-1/2 bg-black rounded-lg px-4 py-3 shadow-lg flex items-center gap-4 text-white">
      <p>
        Previewing <span class="font-medium">{props.pack.name}</span>
      </p>

      <SelectInput
        options={widgetOptions()}
        value={props.widgetName}
        onChange={value => {
          if (value) {
            props.onChange(value, props.presetName);
          }
        }}
      />

      <SelectInput
        options={presetOptions()}
        value={props.presetName}
        onChange={value => {
          if (value) {
            props.onChange(props.widgetName, value);
          }
        }}
      />

      <Button variant="ghost" size="sm" onClick={() => props.onStop()}>
        <IconPlayerPause class="h-4 w-4 mr-2" />
        Stop Preview
      </Button>
    </div>
  );
}
