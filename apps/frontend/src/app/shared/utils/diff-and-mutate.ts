/**
 * Diffs two elements and mutates the first element in-place to match the second
 * element.
 */
export function diffAndMutate(element1: Element, element2: Element) {
  // Compare the tag names of the elements.
  if (element1.tagName !== element2.tagName) {
    // If the tag names are different, replace element1 with element2.
    const parent = element1.parentNode;
    parent?.replaceChild(element2.cloneNode(true), element1);
    return;
  }

  // Compare the attributes of the elements.
  const attributes1 = element1.getAttributeNames();
  const attributes2 = element2.getAttributeNames();

  attributes1.forEach(attr => {
    // Remove attributes from element1 that are not present in element2.
    if (!element2.hasAttribute(attr)) {
      element1.removeAttribute(attr);
    }
  });

  attributes2.forEach(attr => {
    const value = element2.getAttribute(attr)!;

    // Update attributes in element1 to match element2.
    if (!element1.hasAttribute(attr) || element1.getAttribute(attr) !== value) {
      element1.setAttribute(attr, value);
    }
  });

  // TODO: Temporary way to replace child nodes. Should deep diff and mutate
  // for better performance.
  element1.replaceChildren(...Array.from(element2.childNodes));

  return element1;

  // Compare the child nodes of the elements.
  const childNodes1 = Array.from(element1.childNodes);
  const childNodes2 = Array.from(element2.childNodes);

  const length = Math.max(childNodes1.length, childNodes2.length);

  // Recursively diff and mutate child nodes.
  for (let i = 0; i < length; i++) {
    const child1 = childNodes1[i];
    const child2 = childNodes2[i];

    if (child1 && child2) {
      // TODO: This doesn't work for comment or text nodes.
      diffAndMutate(child1 as HTMLElement, child2 as HTMLElement);
    } else if (child1) {
      // Remove extra child nodes from element1.
      element1.removeChild(child1);
    } else if (child2) {
      // Add missing child nodes to element1.
      element1.appendChild(child2.cloneNode(true));
    }
  }
}
