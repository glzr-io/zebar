import { createComputed } from 'solid-js';
import { createStore } from 'solid-js/store';

import { useTemplateEngine } from '~/template-engine';

import {
  TemplateConfig,
  GroupConfig,
  WindowConfig,
  parseConfigSection,
  GroupConfigSchemaP1,
  TemplateConfigSchemaP1,
  WindowConfigSchemaP1,
} from '~/user-config';
import { ElementType } from './shared';

export interface GetParsedElementConfigArgs {
  id: string;
  type: ElementType;
  config: WindowConfig | GroupConfig | TemplateConfig;
  variables: Record<string, unknown>;
}

export function getParsedElementConfig(args: GetParsedElementConfigArgs) {
  const templateEngine = useTemplateEngine();

  const [parsedConfig, setParsedConfig] = createStore(getParsedConfig());

  // Update the store on changes to any provider variables.
  createComputed(() => setParsedConfig(getParsedConfig()));

  /**
   * Get updated store value.
   */
  function getParsedConfig() {
    return parseConfigSection(
      templateEngine,
      { ...args.config, id: args.id },
      getSchemaForElement(args.type),
      args.variables,
    );
  }

  // TODO: Validate in P1 schemas that `template/` and `group/` keys exist.
  function getSchemaForElement(type: ElementType) {
    switch (type) {
      case ElementType.WINDOW:
        return WindowConfigSchemaP1.strip();
      case ElementType.GROUP:
        return GroupConfigSchemaP1.strip();
      case ElementType.TEMPLATE:
        return TemplateConfigSchemaP1.strip();
    }
  }

  return parsedConfig;
}
