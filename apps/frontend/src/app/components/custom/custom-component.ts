import defaultTemplate from './custom-component.njk?raw';
import { createTemplateElement } from '~/shared/template-parsing';
import { CustomComponentConfig } from '~/shared/user-config';

export function CustomComponent(config: CustomComponentConfig): Element {
  return createTemplateElement({
    // Bindings are all user-provided via `template_variables`.
    bindings: () => ({}),
    config: () => config,
    defaultTemplate: () => defaultTemplate,
  });
}
