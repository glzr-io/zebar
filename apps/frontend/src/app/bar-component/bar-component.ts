import { createTemplateElement } from '~/shared/template-parsing';
import { ComponentConfig } from '~/shared/user-config';

export interface BarComponentProps {
  config: ComponentConfig;
}

export function BarComponent(props: BarComponentProps) {
  return createTemplateElement({
    bindings: () => ({}),
    config: () => props.config,
    defaultTemplate: () => '',
  });
}
