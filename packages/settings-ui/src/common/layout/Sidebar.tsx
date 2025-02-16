import { Button, cn, ResizablePanel, Separator } from '@glzr/components';
import {
  IconChevronDown,
  IconChevronsLeft,
  IconHome,
  IconPackage,
  IconWorldSearch,
} from '@tabler/icons-solidjs';
import { A } from '@solidjs/router';
import { createSignal, For } from 'solid-js';

import { SidebarItem } from './SidebarItem';
import { useUserPacks } from '~/common';

export interface SidebarProps {
  initialSize: number;
  onCollapseClick: () => void;
}

export function Sidebar(props: SidebarProps) {
  const [isCollapsed, setIsCollapsed] = createSignal(false);

  const { communityPacks: installedPacks, localPacks } = useUserPacks();

  return (
    <ResizablePanel
      minSize={0.1}
      maxSize={0.2}
      initialSize={props.initialSize}
      collapsible
      onCollapse={size => setIsCollapsed(size === 0)}
      onExpand={() => setIsCollapsed(false)}
      class={cn(
        'overflow-hidden',
        isCollapsed() &&
          'min-w-[50px] transition-all duration-300 ease-in-out',
      )}
    >
      <div class="flex justify-between items-center">
        <img
          src="/resources/logo-128x128.png"
          alt="Zebar logo"
          class={cn(
            'w-8 h-8 m-2 ml-4 transition-all',
            isCollapsed() && 'w-5 h-5',
          )}
        />

        <Button
          onClick={props.onCollapseClick}
          class={cn(
            'mr-2 p-2 text-muted-foreground',
            isCollapsed() && 'hidden',
          )}
          variant="ghost"
        >
          <IconChevronsLeft class="size-4" />
        </Button>
      </div>

      <Separator />

      <SidebarItem
        isCollapsed={isCollapsed()}
        icon={<IconHome class="size-6" />}
        tooltip="Home"
        variant="ghost"
      >
        <A href="/">
          <div class="truncate">Home</div>
        </A>
      </SidebarItem>

      <SidebarItem
        isCollapsed={isCollapsed()}
        icon={<IconWorldSearch class="size-6" />}
        tooltip="Marketplace"
        variant="ghost"
      >
        <A href="/marketplace">
          <div class="truncate">Marketplace</div>
        </A>
      </SidebarItem>

      {!isCollapsed() && (
        <h3 class="px-4 text-xs font-medium text-muted-foreground truncate mt-3">
          Community Packs
        </h3>
      )}

      <For each={installedPacks()}>
        {pack => (
          <SidebarItem
            isCollapsed={isCollapsed()}
            tooltip={pack.name}
            icon={<IconPackage class="size-6" />}
            variant="ghost"
          >
            <div class="flex items-center gap-2 w-full overflow-hidden">
              <div class="truncate">
                <span class="truncate block">{pack.name}</span>
                <span class="truncate block text-xs text-muted-foreground font-normal">
                  {pack.author} • v{pack.version}
                </span>
              </div>
              <IconChevronDown class="size-4 flex-none ml-auto" />
            </div>
          </SidebarItem>
        )}
      </For>

      {!isCollapsed() && (
        <h3 class="px-4 text-xs font-medium text-muted-foreground truncate mt-3">
          Local Packs
        </h3>
      )}

      <For each={localPacks()}>
        {pack => (
          <SidebarItem
            isCollapsed={isCollapsed()}
            icon={<IconPackage class="size-6" />}
            tooltip={pack.name}
            variant="ghost"
          >
            <div class="flex items-center gap-2 w-full overflow-hidden">
              <div class="truncate">{pack.name}</div>
              <IconChevronDown class="size-4 flex-none ml-auto" />
            </div>
          </SidebarItem>
        )}
      </For>
    </ResizablePanel>
  );
}
