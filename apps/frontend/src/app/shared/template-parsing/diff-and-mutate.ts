/**
 * Diffs two elements and mutates the first element in-place to match the second
 * element.
 */
export function diffAndMutate(sourceEl: Element, targetEl: Element) {
  // Compare the tag names of the elements.
  if (sourceEl.tagName !== targetEl.tagName) {
    // If the tag names are different, replace `sourceEl` with `targetEl`.
    const parent = sourceEl.parentNode;
    parent?.replaceChild(targetEl.cloneNode(true), sourceEl);
    return;
  }

  const sourceAttr = sourceEl.getAttributeNames();
  const targetAttr = targetEl.getAttributeNames();

  // Remove attributes from `sourceEl` that are not present in `targetEl`.
  for (const attr of sourceAttr) {
    if (!targetEl.hasAttribute(attr)) {
      sourceEl.removeAttribute(attr);
    }
  }

  for (const attr of targetAttr) {
    const value = targetEl.getAttribute(attr)!;

    // Update attributes in `sourceEl` to match `targetEl`.
    if (!sourceEl.hasAttribute(attr) || sourceEl.getAttribute(attr) !== value) {
      sourceEl.setAttribute(attr, value);
    }
  }

  const sourceChildren = Array.from(sourceEl.childNodes);
  const targetChildren = Array.from(targetEl.childNodes);

  const length = Math.max(sourceChildren.length, targetChildren.length);

  // Recursively diff and mutate child nodes.
  for (let i = 0; i < length; i++) {
    const sourceChild = sourceChildren[i];
    const targetChild = targetChildren[i];

    if (sourceChild && targetChild) {
      if (
        sourceChild.nodeType === Node.ELEMENT_NODE &&
        targetChild.nodeType === Node.ELEMENT_NODE
      ) {
        diffAndMutate(sourceChild as Element, targetChild as Element);
      } else {
        targetEl.replaceChild(sourceChild.cloneNode(true), targetChild);
      }
    } else if (sourceChild) {
      // Remove extra child nodes from `sourceEl`.
      sourceEl.removeChild(sourceChild);
    } else if (targetChild) {
      // Add missing child nodes to `sourceEl`.
      sourceEl.appendChild(targetChild.cloneNode(true));
    }
  }

  return sourceEl;
}
