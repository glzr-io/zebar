import { convertFileSrc } from '@tauri-apps/api/core';
import { join, homeDir } from '@tauri-apps/api/path';

import { createLogger } from '~/utils';
import type { ElementContext } from '../element-context.model';

const logger = createLogger('script-manager');

/**
 * Map of asset paths to promises that resolve to the module.
 */
const modulesByPath: Record<string, Promise<any>> = {};

/**
 * Map of module paths to asset paths.
 */
const modulePathToAssetPath: Record<string, string> = {};

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

  const assetPath =
    modulePathToAssetPath[modulePath] ??
    (modulePathToAssetPath[modulePath] = convertFileSrc(
      await join(await homeDir(), '.glzr/zebar', modulePath),
    ));

  logger.info(
    `Loading script at path '${assetPath}' for function path '${fnPath}'.`,
  );

  const importPromise = import(assetPath);
  modulesByPath[assetPath] = importPromise;

  return importPromise;
}

async function callFn(
  fnPath: string,
  event: Event,
  context: ElementContext,
): Promise<any> {
  const { modulePath, functionName } = parseFnPath(fnPath);
  const assetPath = modulePathToAssetPath[modulePath];
  const foundModule = modulesByPath[assetPath!];

  if (!foundModule) {
    throw new Error(`No script found at function path '${fnPath}'.`);
  }

  return foundModule.then(foundModule => {
    const fn = foundModule[functionName!];

    if (!fn) {
      throw new Error(
        `No function with the name '${functionName}' at function path '${fnPath}'.`,
      );
    }

    return fn(event, context);
  });
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
