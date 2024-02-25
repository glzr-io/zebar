import { createEffect, type Owner } from 'solid-js';
import { createStore } from 'solid-js/store';

import type { KomorebiProviderConfig } from '~/user-config';
import { createProviderListener } from '../create-provider-listener';

export interface KomorebiVariables {
  /**
   * Workspace displayed on the current monitor.
   */
  displayedWorkspace: KomorebiWorkspace;

  /**
   * Workspace that currently has focus (on any monitor).
   */
  focusedWorkspace: KomorebiWorkspace;

  /**
   * Workspaces on the current monitor.
   */
  currentWorkspaces: KomorebiWorkspace[];

  /**
   * Workspaces across all monitors.
   */
  allWorkspaces: KomorebiWorkspace[];

  /**
   * All monitors.
   */
  monitors: KomorebiMonitor[];

  /**
   * Monitor that currently has focus.
   */
  focusedMonitor: KomorebiMonitor;
}

export interface KomorebiMonitor {
  id: number;
  name: string;
  deviceId: string;
  size: KomorebiRect;
  workAreaOffset: number | null;
  workAreaSize: KomorebiRect;
  workspaces: KomorebiWorkspace[];
}

export interface KomorebiWorkspace {
  containerPadding: number;
  floatingWindows: KomorebiWindow[];
  latestLayout: KomorebiRect[];
  layout: KomorebiLayout;
  layoutFlip: KomorebiLayoutFlip | null;
  name: string;
  maximizedWindow: KomorebiWindow | null;
  monocleWindow: KomorebiWindow | null;
  tilingWindows: KomorebiWindow[];
  workspacePadding: number;
}

export interface KomorebiWindow {
  class: string;
  exe: string;
  hwnd: number;
  size: KomorebiRect;
  title: string;
}

export interface KomorebiRect {
  left: number;
  top: number;
  right: number;
  bottom: number;
}

export type KomorebiLayout =
  | 'bsp'
  | 'vertical_stack'
  | 'horizontal_stack'
  | 'ultrawide_vertical_stack'
  | 'rows';

export type KomorebiLayoutFlip = 'horizontal' | 'vertical';

export async function createKomorebiProvider(
  config: KomorebiProviderConfig,
  owner: Owner,
) {
  const providerListener = await createProviderListener<
    KomorebiProviderConfig,
    KomorebiVariables
  >(config, owner);

  const komorebiVariables = createStore({
    workspaces: [],
  });

  createEffect(() => {
    // const { monitors } = providerListener();
    // @ts-ignore
    const monitors = providerListener().monitors;
    console.log('incoming!!!', monitors);
    // const state = JSON.parse(monitors);
    // console.log('state', state);

    // const workspaces = state.workspaces;
  });

  return {
    // get workspaces() {
    //   return providerListener().workspaces;
    // },
  };
}
