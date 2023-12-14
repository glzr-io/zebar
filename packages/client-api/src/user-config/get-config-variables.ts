import { getMonitorPosition } from '~/desktop';

export interface ConfigVariables {
  screen_x: string;
  screen_y: string;
  screen_width: string;
  screen_height: string;
}

// TODO: Remove this in favour of actually reading env/args on startup.
export async function getConfigVariables() {
  const monitorPosition = await getMonitorPosition();

  return {
    screen_x: monitorPosition.x.toString(),
    screen_y: monitorPosition.y.toString(),
    screen_width: monitorPosition.width.toString(),
    screen_height: monitorPosition.height.toString(),
  };
}
