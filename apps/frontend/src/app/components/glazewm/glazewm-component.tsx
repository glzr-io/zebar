import { GlazeWMComponentConfig } from '~/shared/user-config';

export function GlazeWMComponent(props: { config: GlazeWMComponentConfig }) {
  function getBindings() {
    return {
      strings: {
        binding_mode: '',
        root_props: `id="${props.config.id}" class="${props.config.class_name}"`,
        workspaces: [],
      },
      functions: {
        focus_workspace: () => {},
      },
    };
  }

  return document.createElement('div');
}
