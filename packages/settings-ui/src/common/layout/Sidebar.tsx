import { Button, cn, ResizablePanel, Separator } from '@glzr/components';
import {
  IconChevronsLeft,
  IconHome,
  IconWorldSearch,
} from '@tabler/icons-solidjs';
import { createSignal, For } from 'solid-js';

import { useUserPacks } from '~/common';
import { SidebarItem } from './SidebarItem';
import { WidgetPackSidebarItem } from './WidgetPackSidebarItem';

export interface SidebarProps {
  initialSize: number;
  onCollapseClick: () => void;
}

export function Sidebar(props: SidebarProps) {
  const userPacks = useUserPacks();
  const [isCollapsed, setIsCollapsed] = createSignal(false);

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
        href="/"
      >
        <div class="truncate">My widgets</div>
      </SidebarItem>

      <SidebarItem
        isCollapsed={isCollapsed()}
        icon={<IconWorldSearch class="size-6" />}
        tooltip="Marketplace"
        href="/marketplace"
      >
        <div class="truncate">Marketplace</div>
      </SidebarItem>

      {!isCollapsed() && (
        <h3 class="px-4 text-xs font-medium text-muted-foreground truncate mt-3">
          Downloaded packs
        </h3>
      )}

      <For each={userPacks.downloadedPacks()}>
        {pack => (
          <WidgetPackSidebarItem pack={pack} isCollapsed={isCollapsed()} />
        )}
      </For>

      {!isCollapsed() && (
        <h3 class="px-4 text-xs font-medium text-muted-foreground truncate mt-3">
          Custom packs
        </h3>
      )}

      <For each={userPacks.customPacks()}>
        {pack => (
          <WidgetPackSidebarItem pack={pack} isCollapsed={isCollapsed()} />
        )}
      </For>
    </ResizablePanel>
  );
}
