import defaultTemplate from './custom-component.njk?raw';
import { createTemplateElement } from '~/shared/template-parsing';
import { CustomComponentConfig } from '~/shared/user-config';

export function CustomComponent(props: { config: CustomComponentConfig }) {
  return createTemplateElement({
    // Bindings are all user-provided via `template_variables`.
    bindings: () => ({}),
    config: () => props.config,
    defaultTemplate: () => defaultTemplate,
  });
}
