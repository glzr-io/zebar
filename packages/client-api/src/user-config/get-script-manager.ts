import { convertFileSrc } from '@tauri-apps/api/core';
import { join, homeDir } from '@tauri-apps/api/path';
import { createStore } from 'solid-js/store';

import { createLogger } from '~/utils';
import type { ElementContext } from '../element-context.model';

const logger = createLogger('script-manager');

const [modules, setModules] = createStore<Record<string, Promise<any>>>(
  {},
);

/**
 * Abstraction over loading and invoking user-defined scripts.
 */
export function getScriptManager() {
  return {
    loadScript,
    callFn,
  };
}

async function loadScript(path: string): Promise<any> {
  const scriptPath = await join(await homeDir(), '.glzr/zebar', path);
  const scriptAssetPath = convertFileSrc(scriptPath);

  const importPromise = import(scriptAssetPath);
  setModules({ [path]: importPromise });
  return importPromise;
}

async function callFn(
  fnPath: string,
  event: Event,
  context: ElementContext,
): Promise<any> {
  const split = fnPath.split('#');
  const foundModule = modules[split[0]!];

  if (!foundModule) {
    throw new Error('Invalid function path');
  }

  return foundModule.then(m => {
    const fn = m[split[1]!];

    if (!fn) {
      throw new Error('Invalid function path');
    }

    return fn(event, context);
  });
}
