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
  type RunCommandResponse,
  type TilingDirectionChangedEvent,
  type UnlistenFn,
  type Workspace,
  type WorkspaceActivatedEvent,
  type WorkspaceDeactivatedEvent,
  type WorkspaceUpdatedEvent,
} from 'glazewm';
import { z } from 'zod';

import { getMonitors } from '~/desktop';
import { getCoordinateDistance } from '~/utils';
import {
  createBaseProvider,
  type Provider,
} from '../create-base-provider';

export interface GlazeWmProviderConfig {
  type: 'glazewm';
}

const glazeWmProviderConfigSchema = z.object({
  type: z.literal('glazewm'),
});

export type GlazeWmProvider = Provider<
  GlazeWmProviderConfig,
  GlazeWmOutput
>;

export interface GlazeWmOutput {
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
   * Monitor that is nearest to this Zebar widget.
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
   * Invokes a WM command (e.g. `"focus --workspace 1"`).
   *
   * @param command WM command to run (e.g. `"focus --workspace 1"`).
   * @param subjectContainerId (optional) ID of container to use as subject.
   * If not provided, this defaults to the currently focused container.
   * @throws If command fails.
   */
  runCommand(
    command: string,
    subjectContainerId?: string,
  ): Promise<RunCommandResponse>;
}

export function createGlazeWmProvider(
  config: GlazeWmProviderConfig,
): GlazeWmProvider {
  const mergedConfig = glazeWmProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    const monitors = await getMonitors();
    const client = new WmClient();
    let unlistenEvents: null | UnlistenFn = null;

    client.onDisconnect(() =>
      queue.error('Failed to connect to GlazeWM IPC server.'),
    );

    client.onConnect(async () => {
      let state = await getInitialState();
      queue.output(state);

      unlistenEvents ??= await client.subscribeMany(
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

      // TODO: Update state when monitors change.
      // monitors.onChange(async () => {
      //   state = { ...state, ...(await getMonitorState()) };
      //   queue.value(state);
      // });

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
            state = { ...state, bindingModes: e.newBindingModes };
            break;
          }
          case WmEventType.FOCUS_CHANGED: {
            state = { ...state, focusedContainer: e.focusedContainer };
            state = { ...state, ...(await getMonitorState()) };

            const { tilingDirection } =
              await client.queryTilingDirection();
            state = { ...state, tilingDirection };
            break;
          }
          case WmEventType.FOCUSED_CONTAINER_MOVED: {
            state = { ...state, focusedContainer: e.focusedContainer };
            state = { ...state, ...(await getMonitorState()) };
            break;
          }
          case WmEventType.TILING_DIRECTION_CHANGED: {
            state = { ...state, tilingDirection: e.newTilingDirection };
            break;
          }
          case WmEventType.WORKSPACE_ACTIVATED:
          case WmEventType.WORKSPACE_DEACTIVATED:
          case WmEventType.WORKSPACE_UPDATED: {
            state = { ...state, ...(await getMonitorState()) };
            break;
          }
        }

        queue.output(state);
      }

      function runCommand(
        command: string,
        subjectContainerId?: string,
      ): Promise<RunCommandResponse> {
        return client.runCommand(command, subjectContainerId);
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
          runCommand,
        };
      }

      async function getMonitorState() {
        const currentPosition = {
          x: monitors.currentMonitor!.x,
          y: monitors.currentMonitor!.y,
        };

        const { monitors: glazeWmMonitors } = await client.queryMonitors();

        // Get GlazeWM monitor that corresponds to the widget's monitor.
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

        const focusedGlazeWmWorkspace =
          focusedGlazeWmMonitor?.children.find(
            workspace => workspace.hasFocus,
          );

        const displayedGlazeWmWorkspace =
          currentGlazeWmMonitor.children.find(
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
    });

    return () => {
      unlistenEvents?.();
      client.closeConnection();
    };
  });
}
