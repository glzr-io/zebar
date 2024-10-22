import {
  cn,
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
  IconChevronDown,
  IconFile,
} from '@glzr/components';
import { createMemo, For } from 'solid-js';

import { WidgetConfig } from 'zebar';

export interface WidgetConfigTreeProps {
  configs: Record<string, WidgetConfig>;
  selectedConfig: WidgetConfig | null;
  selectedConfigPath: string | null;
  onSelect: (configPath: string) => void;
}

export function WidgetConfigTree(props: WidgetConfigTreeProps) {
  const configTree = createMemo(() => {
    const tree: Record<string, string[]> = {};

    Object.keys(props.configs)
      .sort()
      .forEach(configPath => {
        const folder = configPath.split(/[/\\]/).at(-2);
        tree[folder] = [...(tree[folder] ?? []), configPath];
      });

    return tree;
  });

  return (
    <div class="border-r p-4 h-full w-[clamp(200px,20vw,300px)]">
      <h2 class="text-lg font-semibold mb-2">Widget configs</h2>
      <div class="space-y-1">
        <For each={Object.entries(configTree())}>
          {([folder, configPaths]) => (
            <Collapsible defaultOpen>
              <CollapsibleTrigger class="flex items-center space-x-2 px-2 py-1 w-full text-left">
                <IconChevronDown class="h-3 w-3" />
                <span>{folder}</span>
              </CollapsibleTrigger>

              <CollapsibleContent class="pl-4">
                {configPaths.map(configPath => (
                  <div
                    class={cn(
                      'flex items-center space-x-2 py-1 rounded-md cursor-pointer',
                      props.selectedConfigPath === configPath &&
                        'bg-accent',
                    )}
                    onClick={() => props.onSelect(configPath)}
                  >
                    <IconFile class="h-4 w-4" />
                    <span>{configPath.split(/[/\\]/).at(-1)}</span>
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
