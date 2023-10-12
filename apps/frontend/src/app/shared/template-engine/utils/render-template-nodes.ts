import { TemplateNode } from '../types/template-node.model';

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
