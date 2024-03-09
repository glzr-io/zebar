import { convertFileSrc } from '@tauri-apps/api/core';
import { join, homeDir } from '@tauri-apps/api/path';
import {
  type Owner,
  createEffect,
  createSignal,
  runWithOwner,
} from 'solid-js';
import { createStore } from 'solid-js/store';

import { createLogger } from '~/utils';

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
  console.log('path', scriptPath, scriptAssetPath);
  const importPromise = import(scriptAssetPath);

  const imgPath = await join(await homeDir(), '.glzr/zebar', 'out.png');
  const imgAssetPath = convertFileSrc(imgPath);
  const img = new Image();
  img.src = imgAssetPath;
  document.body.appendChild(img);

  setModules({ [path]: importPromise });
  return importPromise;
}

async function callFn(fnPath: string): Promise<any> {
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

    return fn();
  });
}
