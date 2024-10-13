import {
  cn,
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
  IconChevronDown,
  IconFile,
  IconFolder,
} from '@glzr/components';
import { createMemo, createSignal, For } from 'solid-js';

import { WidgetConfigEntry } from './WidgetSettings';

export interface WidgetConfigTreeProps {
  configs: WidgetConfigEntry[];
  onSelect: (entry: WidgetConfigEntry) => void;
}

export function WidgetConfigTree(props: WidgetConfigTreeProps) {
  const configTree = createMemo(() => {
    const tree: Record<string, WidgetConfigEntry[]> = {};

    props.configs.forEach(config => {
      const folder = config.configPath.split(/[/\\]/).at(-2);
      tree[folder] = [...(tree[folder] ?? []), config];
    });

    return tree;
  });

  const [selectedFile, setSelectedFile] = createSignal<string | null>(
    null,
  );

  return (
    <div class="border p-4">
      <h2 class="text-lg font-semibold mb-2">Widget configs</h2>
      <div class="space-y-1">
        <For each={Object.entries(configTree())}>
          {([folder, configs]) => (
            <Collapsible defaultOpen>
              <CollapsibleTrigger class="flex items-center space-x-2 px-2 py-1 w-full text-left">
                <IconChevronDown class="h-3 w-3" />
                <span>{folder}</span>
              </CollapsibleTrigger>
              <CollapsibleContent class="pl-4">
                {configs.map(config => (
                  <div
                    class={cn(
                      'flex items-center space-x-2 py-1 rounded-md cursor-pointer',
                      selectedFile() === config.configPath && 'bg-accent',
                    )}
                    onClick={() => setSelectedFile(config.configPath)}
                  >
                    <IconFile class="h-4 w-4" />
                    <span>{config.configPath.split(/[/\\]/).at(-1)}</span>
                  </div>
                ))}
              </CollapsibleContent>
            </Collapsible>
          )}
        </For>
      </div>

      {selectedFile && (
        <div class="mt-4 p-2 bg-muted rounded-md">
          Selected: {selectedFile()}
        </div>
      )}
    </div>
  );
}
