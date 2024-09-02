import {
  TilingDirection,
  WmClient,
  WmEventType,
  type BindingModeConfig,
  type BindingModesChangedEvent,
  type Container,
  type FocusChangedEvent,
  type FocusedContainerMovedEvent,
  type Monitor,
  type TilingDirectionChangedEvent,
  type Workspace,
  type WorkspaceActivatedEvent,
  type WorkspaceDeactivatedEvent,
  type WorkspaceUpdatedEvent,
} from 'glazewm';
import { createEffect, on, runWithOwner, type Owner } from 'solid-js';
import { createStore } from 'solid-js/store';
import { z } from 'zod';

import { getMonitors } from '~/desktop';
import { getCoordinateDistance } from '~/utils';

export interface GlazeWmProviderConfig {
  type: 'glazewm';
}

const GlazeWmProviderConfigSchema = z.object({
  type: z.literal('glazewm'),
});

export interface GlazeWmProvider {
  /**
   * Workspace displayed on the current monitor.
   */
  displayedWorkspace: Workspace;

  /**
   * Workspace that currently has focus (on any monitor).
   */
  focusedWorkspace: Workspace;

  /**
   * Workspaces on the current monitor.
   */
  currentWorkspaces: Workspace[];

  /**
   * Workspaces across all monitors.
   */
  allWorkspaces: Workspace[];

  /**
   * All monitors.
   */
  allMonitors: Monitor[];

  /**
   * Monitor that currently has focus.
   */
  focusedMonitor: Monitor;

  /**
   * Monitor that is nearest to this Zebar window.
   */
  currentMonitor: Monitor;

  /**
   * Container that currently has focus (on any monitor).
   */
  focusedContainer: Container;

  /**
   * Tiling direction of the focused container.
   */
  tilingDirection: TilingDirection;

  /**
   * Active binding modes;
   */
  bindingModes: BindingModeConfig[];

  /**
   * Focus a workspace by name.
   */
  focusWorkspace(name: string): void;

  /**
   * Toggle tiling direction.
   */
  toggleTilingDirection(): void;
}

export async function createGlazeWmProvider(
  _: GlazeWmProviderConfig,
  owner: Owner,
): Promise<GlazeWmProvider> {
  const monitors = await getMonitors();
  const client = new WmClient();

  const [glazeWmVariables, setGlazeWmVariables] = createStore(
    await getInitialState(),
  );

  await client.subscribeMany(
    [
      WmEventType.BINDING_MODES_CHANGED,
      WmEventType.FOCUS_CHANGED,
      WmEventType.FOCUSED_CONTAINER_MOVED,
      WmEventType.TILING_DIRECTION_CHANGED,
      WmEventType.WORKSPACE_ACTIVATED,
      WmEventType.WORKSPACE_DEACTIVATED,
      WmEventType.WORKSPACE_UPDATED,
    ],
    onEvent,
  );

  runWithOwner(owner, () => {
    createEffect(
      on(
        () => monitors.currentMonitor,
        async () => setGlazeWmVariables({ ...(await getMonitorState()) }),
      ),
    );
  });

  async function onEvent(
    e:
      | BindingModesChangedEvent
      | FocusChangedEvent
      | FocusedContainerMovedEvent
      | TilingDirectionChangedEvent
      | WorkspaceActivatedEvent
      | WorkspaceDeactivatedEvent
      | WorkspaceUpdatedEvent,
  ) {
    switch (e.eventType) {
      case WmEventType.BINDING_MODES_CHANGED: {
        setGlazeWmVariables({ bindingModes: e.newBindingModes });
        break;
      }
      case WmEventType.FOCUS_CHANGED: {
        setGlazeWmVariables({ focusedContainer: e.focusedContainer });
        setGlazeWmVariables({ ...(await getMonitorState()) });

        const { tilingDirection } = await client.queryTilingDirection();
        setGlazeWmVariables({ tilingDirection });
        break;
      }
      case WmEventType.FOCUSED_CONTAINER_MOVED: {
        setGlazeWmVariables({ focusedContainer: e.focusedContainer });
        setGlazeWmVariables({ ...(await getMonitorState()) });
        break;
      }
      case WmEventType.TILING_DIRECTION_CHANGED: {
        setGlazeWmVariables({ tilingDirection: e.newTilingDirection });
        break;
      }
      case WmEventType.WORKSPACE_ACTIVATED:
      case WmEventType.WORKSPACE_DEACTIVATED:
      case WmEventType.WORKSPACE_UPDATED: {
        setGlazeWmVariables({ ...(await getMonitorState()) });
        break;
      }
    }
  }

  async function getInitialState() {
    const { focused: focusedContainer } = await client.queryFocused();
    const { bindingModes } = await client.queryBindingModes();
    const { tilingDirection } = await client.queryTilingDirection();

    return {
      ...(await getMonitorState()),
      focusedContainer,
      tilingDirection,
      bindingModes,
    };
  }

  async function getMonitorState() {
    const currentPosition = {
      x: monitors.currentMonitor!.x,
      y: monitors.currentMonitor!.y,
    };

    const { monitors: glazeWmMonitors } = await client.queryMonitors();

    // Get GlazeWM monitor that corresponds to the Zebar window's monitor.
    const currentGlazeWmMonitor = glazeWmMonitors.reduce((a, b) =>
      getCoordinateDistance(currentPosition, a) <
      getCoordinateDistance(currentPosition, b)
        ? a
        : b,
    );

    const focusedGlazeWmMonitor = glazeWmMonitors.find(
      monitor => monitor.hasFocus,
    );

    const allGlazeWmWorkspaces = glazeWmMonitors.flatMap(
      monitor => monitor.children,
    );

    const focusedGlazeWmWorkspace = focusedGlazeWmMonitor?.children.find(
      workspace => workspace.hasFocus,
    );

    const displayedGlazeWmWorkspace = currentGlazeWmMonitor.children.find(
      workspace => workspace.isDisplayed,
    );

    return {
      displayedWorkspace: displayedGlazeWmWorkspace!,
      focusedWorkspace: focusedGlazeWmWorkspace!,
      currentWorkspaces: currentGlazeWmMonitor.children,
      allWorkspaces: allGlazeWmWorkspaces,
      focusedMonitor: focusedGlazeWmMonitor!,
      currentMonitor: currentGlazeWmMonitor,
      allMonitors: glazeWmMonitors,
    };
  }

  function focusWorkspace(name: string) {
    client.runCommand(`focus --workspace ${name}`);
  }

  function toggleTilingDirection() {
    client.runCommand('toggle-tiling-direction');
  }

  return {
    get displayedWorkspace() {
      return glazeWmVariables.displayedWorkspace;
    },
    get focusedWorkspace() {
      return glazeWmVariables.focusedWorkspace;
    },
    get currentWorkspaces() {
      return glazeWmVariables.currentWorkspaces;
    },
    get allWorkspaces() {
      return glazeWmVariables.allWorkspaces;
    },
    get allMonitors() {
      return glazeWmVariables.allMonitors;
    },
    get focusedMonitor() {
      return glazeWmVariables.focusedMonitor;
    },
    get currentMonitor() {
      return glazeWmVariables.currentMonitor;
    },
    get focusedContainer() {
      return glazeWmVariables.focusedContainer;
    },
    get tilingDirection() {
      return glazeWmVariables.tilingDirection;
    },
    get bindingModes() {
      return glazeWmVariables.bindingModes;
    },
    focusWorkspace,
    toggleTilingDirection,
  };
}
