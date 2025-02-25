import {
  buttonVariants,
  cn,
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@glzr/components';
import { A } from '@solidjs/router';
import { type JSX, Show } from 'solid-js';

export interface SidebarItemProps {
  isCollapsed: boolean;
  icon: JSX.Element;
  tooltip: string;
  children: JSX.Element;
  href: string;
  onClick?: (e: MouseEvent) => void;
}

/**
 * Sidebar item component.
 *
 * Shows only the icon and a tooltip on hover when collapsed.
 */
export function SidebarItem(props: SidebarItemProps) {
  const ExpandedButton = () => (
    <A
      href={props.href}
      onClick={props.onClick}
      activeClass="bg-accent text-accent-foreground"
      end={true}
      class={cn(
        buttonVariants({ variant: 'ghost' }),
        'flex justify-start min-w-0 m-2 pl-2',
      )}
    >
      <div class="mr-2">{props.icon}</div>
      <div class="flex-1 min-w-0">{props.children}</div>
    </A>
  );

  const CollapsedButton = () => (
    <Tooltip openDelay={0} closeDelay={0} placement="right">
      <TooltipTrigger
        as={A}
        href={props.href}
        onClick={props.onClick}
        activeClass="bg-accent text-accent-foreground"
        end={true}
        class={cn(
          buttonVariants({
            variant: 'ghost',
            size: 'icon',
          }),
          'm-2',
        )}
      >
        {props.icon}
        <span class="sr-only">{props.tooltip}</span>
      </TooltipTrigger>

      <TooltipContent>{props.tooltip}</TooltipContent>
    </Tooltip>
  );

  return (
    <Show when={props.isCollapsed} fallback={<ExpandedButton />}>
      <CollapsedButton />
    </Show>
  );
}
