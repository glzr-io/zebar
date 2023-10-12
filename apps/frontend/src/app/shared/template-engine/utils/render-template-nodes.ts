import { TemplateNode } from '../types';

export function renderTemplateNodes(
  nodes: TemplateNode[],
  globalContext: Record<string, unknown>,
) {
  const context = {
    global: globalContext,
    local: {},
  };

  return '';
}
