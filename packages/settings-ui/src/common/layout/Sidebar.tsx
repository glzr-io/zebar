import { Button, cn, ResizablePanel, Separator } from '@glzr/components';
import {
  IconChevronDown,
  IconChevronsLeft,
  IconHome,
  IconPackage,
  IconWorldSearch,
  IconChevronRight,
} from '@tabler/icons-solidjs';
import { A, useLocation } from '@solidjs/router';
import { createSignal, For, Show } from 'solid-js';

import { SidebarItem } from './SidebarItem';
import { useUserPacks } from '~/common';

export interface SidebarProps {
  initialSize: number;
  onCollapseClick: () => void;
}

export function Sidebar(props: SidebarProps) {
  const location = useLocation();
  const [isCollapsed, setIsCollapsed] = createSignal(false);
  const [expandedPacks, setExpandedPacks] = createSignal<
    Record<string, boolean>
  >({});

  const { downloadedPacks, localPacks } = useUserPacks();

  function togglePackExpanded(packId: string, e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();
    setExpandedPacks(prev => ({
      ...prev,
      [packId]: !prev[packId],
    }));
  }

  function isCurrentRoute(path: string) {
    return (
      location.pathname === path ||
      location.pathname.startsWith(path + '/')
    );
  }

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
        <div class="truncate">Browse Community</div>
      </SidebarItem>

      {!isCollapsed() && (
        <h3 class="px-4 text-xs font-medium text-muted-foreground truncate mt-3">
          Downloads
        </h3>
      )}

      <For each={downloadedPacks()}>
        {pack => (
          <>
            <SidebarItem
              isCollapsed={isCollapsed()}
              tooltip={pack.name}
              icon={<IconPackage class="size-6" />}
              href={`/packs/${pack.id}`}
            >
              <div class="flex items-center gap-2 w-full overflow-hidden">
                <div class="truncate flex-1">
                  <span class="truncate block">{pack.name}</span>
                  <span class="truncate block text-xs text-muted-foreground font-normal">
                    {pack.author} • v{pack.version}
                  </span>
                </div>
                <Button
                  variant="ghost"
                  size="icon"
                  class="size-6 p-0 ml-auto flex-none"
                  onClick={e => togglePackExpanded(pack.id, e)}
                >
                  <Show
                    when={expandedPacks()[pack.id]}
                    fallback={<IconChevronRight class="size-4" />}
                  >
                    <IconChevronDown class="size-4" />
                  </Show>
                </Button>
              </div>
            </SidebarItem>

            <Show when={!isCollapsed() && expandedPacks()[pack.id]}>
              <div class="ml-6 mr-2">
                <For each={pack.widgetConfigs}>
                  {config => (
                    <A
                      href={`/packs/${pack.id}/${config.value.name}`}
                      class="block"
                    >
                      <div
                        class={cn(
                          'text-sm py-1.5 px-2 rounded-md truncate',
                          isCurrentRoute(
                            `/packs/${pack.id}/${config.value.name}`,
                          )
                            ? 'bg-accent text-accent-foreground'
                            : 'hover:bg-accent/50',
                        )}
                      >
                        {config.value.name}
                      </div>
                    </A>
                  )}
                </For>
              </div>
            </Show>
          </>
        )}
      </For>

      {!isCollapsed() && (
        <h3 class="px-4 text-xs font-medium text-muted-foreground truncate mt-3">
          Personal
        </h3>
      )}

      <For each={localPacks()}>
        {pack => (
          <>
            <SidebarItem
              isCollapsed={isCollapsed()}
              icon={<IconPackage class="size-6" />}
              tooltip={pack.name}
              href={`/packs/${pack.id}`}
            >
              <div class="flex items-center gap-2 w-full overflow-hidden">
                <div class="truncate flex-1">{pack.name}</div>
                <Button
                  variant="ghost"
                  size="icon"
                  class="size-6 p-0 ml-auto flex-none"
                  onClick={e => togglePackExpanded(pack.id, e)}
                >
                  <Show
                    when={expandedPacks()[pack.id]}
                    fallback={<IconChevronRight class="size-4" />}
                  >
                    <IconChevronDown class="size-4" />
                  </Show>
                </Button>
              </div>
            </SidebarItem>

            <Show when={!isCollapsed() && expandedPacks()[pack.id]}>
              <div class="ml-6 mr-2">
                <For each={pack.widgetConfigs}>
                  {config => (
                    <A
                      href={`/packs/${pack.id}/${config.value.name}`}
                      class="block"
                    >
                      <div
                        class={cn(
                          'text-sm py-1.5 px-2 rounded-md truncate',
                          isCurrentRoute(
                            `/packs/${pack.id}/${config.value.name}`,
                          )
                            ? 'bg-accent text-accent-foreground'
                            : 'hover:bg-accent/50',
                        )}
                      >
                        {config.value.name}
                      </div>
                    </A>
                  )}
                </For>
              </div>
            </Show>
          </>
        )}
      </For>
    </ResizablePanel>
  );
}
