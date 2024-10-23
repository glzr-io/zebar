import {
  cn,
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
  IconChevronDown,
  IconDiamonds,
  IconFile,
} from '@glzr/components';
import { createMemo, For } from 'solid-js';
import { WidgetConfig } from 'zebar';

export interface WidgetConfigSidebarProps {
  configs: Record<string, WidgetConfig>;
  selectedConfig: WidgetConfig | null;
  selectedConfigPath: string | null;
  onSelect: (configPath: string) => void;
}

export function WidgetConfigSidebar(props: WidgetConfigSidebarProps) {
  const configTree = createMemo(() => {
    const tree: Record<string, Record<string, WidgetConfig>> = {};

    Object.keys(props.configs)
      .sort()
      .forEach(configPath => {
        const folder = configPath.split(/[/\\]/).at(-2);

        tree[folder] = {
          ...(tree[folder] ?? {}),
          [configPath]: props.configs[configPath],
        };
      });

    return tree;
  });

  return (
    <div class="border-r p-4 h-full w-[clamp(200px,20vw,300px)]">
      <h2 class="text-lg font-semibold mb-2">Widget configs</h2>
      <div class="space-y-1">
        <For each={Object.entries(configTree())}>
          {([folder, configs]) => (
            <Collapsible defaultOpen>
              <CollapsibleTrigger class="flex items-center space-x-2 py-1 w-full text-left">
                <IconChevronDown class="size-3" />
                <span class="truncate">{folder}</span>
              </CollapsibleTrigger>

              <CollapsibleContent>
                <For each={Object.entries(configs)}>
                  {([configPath]) => (
                    <div
                      class={cn(
                        'flex items-center pl-3 py-1 space-x-2 rounded-sm cursor-pointer',
                        props.selectedConfigPath === configPath &&
                          'bg-accent',
                      )}
                      onClick={() => props.onSelect(configPath)}
                    >
                      <IconDiamonds class="size-3 text-sky-600" />
                      <span class="truncate">
                        {configPath
                          .split(/[/\\]/)
                          .at(-1)
                          .replace('.zebar.json', '')}
                      </span>
                    </div>
                  )}
                </For>
              </CollapsibleContent>
            </Collapsible>
          )}
        </For>
      </div>
    </div>
  );
}
