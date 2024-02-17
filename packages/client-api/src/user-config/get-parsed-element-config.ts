import { type Owner, createComputed, runWithOwner } from 'solid-js';
import { createStore } from 'solid-js/store';

import type { ElementContext } from '~/element-context.model';
import { ElementType } from '~/element-type.model';
import { TemplateError, getTemplateEngine } from '~/template-engine';
import {
  GroupConfigSchemaP1,
  TemplateConfigSchema,
  WindowConfigSchemaP1,
  parseWithSchema,
} from '~/user-config';
import type { PickPartial } from '~/utils';

export function getParsedElementConfig(
  elementContext: PickPartial<ElementContext, 'parsedConfig'>,
  owner: Owner,
) {
  const templateEngine = getTemplateEngine();

  const [parsedConfig, setParsedConfig] = createStore(getParsedConfig());

  // Update the store on changes to any provider variables.
  runWithOwner(owner, () => {
    createComputed(() => setParsedConfig(getParsedConfig()));
  });

  /**
   * Get updated store value.
   */
  function getParsedConfig() {
    const config = {
      ...(elementContext.rawConfig as Record<string, unknown>),
      id: elementContext.id,
    };

    const schema = getSchemaForElement(elementContext.type);

    const newConfigEntries = Object.entries(config).map(([key, value]) => {
      // If value is not a string, then it can't contain any templating syntax.
      if (typeof value !== 'string') {
        return [key, value];
      }

      // Run the value through the templating engine.
      try {
        const rendered = templateEngine.render(
          value,
          elementContext.providers,
        );

        return [key, rendered];
      } catch (err) {
        if (!(err instanceof TemplateError)) {
          throw err;
        }

        const { message, templateIndex } = err;

        throw new Error(
          `Property '${key}' in config isn't valid.\n\n` +
            'Syntax error at:\n' +
            `...${value.slice(templateIndex - 30, templateIndex)} << \n\n` +
            `⚠️ ${message}`,
        );
      }
    });

    // TODO: Add logging for updated config here.
    const newConfig = Object.fromEntries(newConfigEntries);

    return parseWithSchema(schema, newConfig);
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
      return TemplateConfigSchema.strip();
  }
}
