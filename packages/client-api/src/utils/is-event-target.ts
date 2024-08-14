export function isEventTarget(obj: any): obj is EventTarget {
  return obj && typeof obj.addEventListener === 'function';
}
