/**
 * Get distance between two points.
 */
export function getCoordinateDistance(
  pointA: { x: number; y: number },
  pointB: { x: number; y: number },
) {
  return Math.sqrt(
    Math.pow(pointB.x - pointA.x, 2) + Math.pow(pointB.y - pointA.y, 2),
  );
}
