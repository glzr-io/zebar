import { Owner, createComputed, runWithOwner } from 'solid-js';
import { createStore } from 'solid-js/store';

import { ElementType } from '~/element-type.model';
import { TemplateError, getTemplateEngine } from '~/template-engine';
import {
  TemplateConfig,
  GroupConfig,
  WindowConfig,
  GroupConfigSchemaP1,
  TemplateConfigSchemaP1,
  WindowConfigSchemaP1,
  TemplatePropertyError,
} from '~/user-config';

export interface GetParsedElementConfigArgs {
  id: string;
  type: ElementType;
  rawConfig: WindowConfig | GroupConfig | TemplateConfig;
  providers: Record<string, unknown>;
  owner: Owner;
}

export function getParsedElementConfig(args: GetParsedElementConfigArgs) {
  const templateEngine = getTemplateEngine();

  const [parsedConfig, setParsedConfig] = createStore(getParsedConfig());

  // Update the store on changes to any provider variables.
  runWithOwner(args.owner, () => {
    createComputed(() => setParsedConfig(getParsedConfig()));
  });

  /**
   * Get updated store value.
   */
  function getParsedConfig() {
    const config = { ...args.rawConfig, id: args.id };
    const schema = getSchemaForElement(args.type);

    const newConfigEntries = Object.entries(config).map(([key, value]) => {
      // If value is not a string, then it can't contain any templating syntax.
      if (typeof value !== 'string') {
        return [key, value];
      }

      // Run the value through the templating engine.
      try {
        const rendered = templateEngine.render(value, args.providers);
        return [key, rendered];
      } catch (err) {
        // Re-throw error as `TemplatePropertyError`.
        throw err instanceof TemplateError
          ? new TemplatePropertyError(
              err.message,
              key,
              value,
              err.templateIndex,
            )
          : err;
      }
    });

    // TODO: Add logging for updated config here.
    const newConfig = Object.fromEntries(newConfigEntries);

    return schema.parse(newConfig);
  }

  return parsedConfig;
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
