import { convertFileSrc } from '@tauri-apps/api/core';
import { join, homeDir } from '@tauri-apps/api/path';

import { createLogger } from '~/utils';
import type { ElementContext } from '../element-context.model';

const logger = createLogger('script-manager');

/**
 * Map of module paths to asset paths.
 */
const assetPathCache: Record<string, string> = {};

/**
 * Map of asset paths to promises that resolve to the module.
 */
const moduleCache: Record<string, Promise<any>> = {};

/**
 * Abstraction over loading and invoking user-defined scripts.
 */
export function getScriptManager() {
  return {
    loadScriptForFn,
    callFn,
  };
}

async function loadScriptForFn(fnPath: string): Promise<any> {
  const { modulePath } = parseFnPath(fnPath);
  return resolveModule(modulePath);
}

async function callFn(
  fnPath: string,
  event: Event,
  context: ElementContext,
): Promise<any> {
  const { modulePath, functionName } = parseFnPath(fnPath);
  const foundModule = await resolveModule(modulePath);
  const fn = foundModule[functionName];

  if (!fn) {
    throw new Error(
      `No function with the name '${functionName}' at function path '${fnPath}'.`,
    );
  }

  return fn(event, context);
}

async function resolveModule(modulePath: string): Promise<any> {
  const assetPath = await getAssetPath(modulePath);
  const foundModule = moduleCache[assetPath];

  if (foundModule) {
    return foundModule;
  }

  logger.info(`Loading script at path '${assetPath}'.`);
  return (moduleCache[assetPath] = import(/* @vite-ignore */ assetPath));
}

/**
 * Converts user-defined path to a URL that can be loaded by the webview.
 */
async function getAssetPath(modulePath: string): Promise<string> {
  const foundAssetPath = assetPathCache[modulePath];

  if (foundAssetPath) {
    return foundAssetPath;
  }

  return (assetPathCache[modulePath] = convertFileSrc(
    await join(await homeDir(), '.glzr/zebar', modulePath),
  ));
}

function parseFnPath(fnPath: string): {
  modulePath: string;
  functionName: string;
} {
  const [modulePath, functionName] = fnPath.split('#');

  // Should never been thrown, as the path is validated during config
  // deserialization.
  if (!modulePath || !functionName) {
    throw new Error(`Invalid function path '${fnPath}'.`);
  }

  return { modulePath, functionName };
}
