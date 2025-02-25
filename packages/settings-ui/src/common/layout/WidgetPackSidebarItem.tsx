import { Button } from '@glzr/components';
import {
  IconChevronDown,
  IconChevronRight,
  IconPackage,
} from '@tabler/icons-solidjs';
import { A } from '@solidjs/router';
import { createSignal, For, Show } from 'solid-js';

import { WidgetPack } from '~/common';
import { SidebarItem } from './SidebarItem';

export interface WidgetPackSidebarItemProps {
  pack: WidgetPack;
  isCollapsed: boolean;
}

export function WidgetPackSidebarItem(props: WidgetPackSidebarItemProps) {
  const [isExpanded, setIsExpanded] = createSignal(false);

  function toggleExpanded(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();
    setIsExpanded(prev => !prev);
  }

  return (
    <>
      <SidebarItem
        isCollapsed={props.isCollapsed}
        tooltip={props.pack.name}
        icon={<IconPackage class="size-6" />}
        href={`/packs/${props.pack.id}`}
      >
        <div class="flex items-center gap-2 w-full overflow-hidden">
          <div class="truncate flex-1">
            {props.pack.type === 'marketplace' && (
              <>
                <span class="truncate block">{props.pack.name}</span>
                <span class="truncate block text-xs text-muted-foreground font-normal">
                  {props.pack.author} • v{props.pack.version}
                </span>
              </>
            )}
            {props.pack.type !== 'marketplace' && (
              <div class="truncate">{props.pack.name}</div>
            )}
          </div>
          <Button
            variant="ghost"
            size="icon"
            class="size-6 p-0 ml-auto flex-none"
            onClick={toggleExpanded}
          >
            <Show
              when={isExpanded()}
              fallback={<IconChevronRight class="size-4" />}
            >
              <IconChevronDown class="size-4" />
            </Show>
          </Button>
        </div>
      </SidebarItem>

      <Show when={!props.isCollapsed && isExpanded()}>
        <div class="ml-6 mr-2">
          <For each={props.pack.widgetConfigs}>
            {config => (
              <A
                href={`/packs/${props.pack.id}/${config.value.name}`}
                class="block text-sm py-1.5 px-2 rounded-md truncate"
                activeClass="bg-accent text-accent-foreground"
                inactiveClass="hover:bg-accent/50"
              >
                {config.value.name}
              </A>
            )}
          </For>
        </div>
      </Show>
    </>
  );
}
