import {
  cn,
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
  IconChevronDown,
  IconFile,
} from '@glzr/components';
import { createMemo, For } from 'solid-js';

import { WidgetConfigEntry } from './WidgetSettings';

export interface WidgetConfigTreeProps {
  configEntries: WidgetConfigEntry[];
  selectedEntry: WidgetConfigEntry | null;
  onSelect: (configPath: string) => void;
}

export function WidgetConfigTree(props: WidgetConfigTreeProps) {
  const configTree = createMemo(() => {
    const tree: Record<string, WidgetConfigEntry[]> = {};

    props.configEntries.forEach(config => {
      const folder = config.configPath.split(/[/\\]/).at(-2);
      tree[folder] = [...(tree[folder] ?? []), config];
    });

    return tree;
  });

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
                      props.selectedEntry?.configPath ===
                        config.configPath && 'bg-accent',
                    )}
                    onClick={() => props.onSelect(config.configPath)}
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
    </div>
  );
}
