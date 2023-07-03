function diffAndMutate(element1: HTMLElement, element2: HTMLElement) {
  // Compare the tag names of the elements.
  if (element1.tagName !== element2.tagName) {
    // If the tag names are different, replace element1 with element2.
    const parent = element1.parentNode;
    parent?.replaceChild(element2.cloneNode(true), element1);
    return;
  }

  // Compare the attributes of the elements.
  const attributes1 = Array.from(element1.attributes);
  const attributes2 = Array.from(element2.attributes);

  attributes1.forEach(attr => {
    const name = attr.name;

    // Remove attributes from element1 that are not present in element2.
    if (!element2.hasAttribute(name)) {
      element1.removeAttribute(name);
    }
  });

  attributes2.forEach(attr => {
    const name = attr.name;
    const value = attr.value;

    // Update attributes in element1 to match element2.
    if (!element1.hasAttribute(name) || element1.getAttribute(name) !== value) {
      element1.setAttribute(name, value);
    }
  });

  // Compare the child nodes of the elements.
  const childNodes1 = Array.from(element1.childNodes);
  const childNodes2 = Array.from(element2.childNodes);

  const length = Math.max(childNodes1.length, childNodes2.length);

  // Recursively diff and mutate child nodes.
  for (let i = 0; i < length; i++) {
    const child1 = childNodes1[i];
    const child2 = childNodes2[i];

    if (child1 && child2) {
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
