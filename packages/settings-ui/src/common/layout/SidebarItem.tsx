import {
  buttonVariants,
  cn,
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@glzr/components';
import { type JSX, Show } from 'solid-js';

export interface SidebarItemProps {
  isCollapsed: boolean;
  icon: JSX.Element;
  tooltip: string;
  children: JSX.Element;
  variant: 'default' | 'ghost';
  isActive?: boolean;
}

/**
 * Sidebar item component.
 *
 * Shows only the icon and a tooltip on hover when collapsed.
 */
export function SidebarItem(props: SidebarItemProps) {
  const ExpandedButton = () => (
    <div
      class={cn(
        buttonVariants({ variant: props.variant }),
        'flex justify-start min-w-0 m-2 pl-2',
        props.isActive && 'bg-accent text-accent-foreground',
      )}
    >
      <div class="mr-2">{props.icon}</div>
      <div class="flex-1 min-w-0">{props.children}</div>
    </div>
  );

  const CollapsedButton = () => (
    <Tooltip openDelay={0} closeDelay={0} placement="right">
      <TooltipTrigger
        class={cn(
          buttonVariants({
            variant: props.variant,
            size: 'icon',
          }),
          'm-2',
          props.isActive && 'bg-accent text-accent-foreground',
        )}
      >
        {props.icon}
        <span class="sr-only">{props.tooltip}</span>
      </TooltipTrigger>

      <TooltipContent>{props.tooltip}</TooltipContent>
    </Tooltip>
  );

  return (
    <Show
      when={props.isCollapsed && props.tooltip}
      fallback={<ExpandedButton />}
    >
      <CollapsedButton />
    </Show>
  );
}
