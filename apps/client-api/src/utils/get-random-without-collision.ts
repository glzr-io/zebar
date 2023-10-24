/**
 * Create a random string that doesn't exist as a key in given object.
 *
 * @param map Map to check whether key exists.
 */
export function getRandomWithoutCollision(map: Record<string, unknown>) {
  let random: string | undefined;

  do {
    random = `a${Math.random().toString().slice(2)}`;
  } while (map[random]);

  return random;
}
