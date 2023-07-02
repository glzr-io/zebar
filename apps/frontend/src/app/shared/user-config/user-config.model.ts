export interface UserConfig {
  general: GeneralConfig;
  [key: `bars/${string}`]: BarConfig;
}

export interface GeneralConfig {
  position_x: string;
  position_y: string;
  width: string;
  height: string;
  opacity: number;
  enable_devtools: boolean;
  enable_default_styles: boolean;
  global_style: string;
  global_stylesheet_path: string;
}

export interface BarConfig {
  style: string;
  template: string;
  components_left: ComponentContainerConfig;
  components_middle: ComponentContainerConfig;
  components_right: ComponentContainerConfig;
}

export interface ComponentContainerConfig {
  style: string;
  template_variables: Record<string, string | ScriptVariableConfig>;
  template_commands: Record<string, string>;
  template: string;
  components: ComponentConfig[];
}

export interface ScriptVariableConfig {
  source: 'script';
  script_path: string;
  script_args: string;
  refresh_interval_ms: number;
}

export interface ComponentConfig {
  type: string;
  style: string;
  template_variables: Record<string, string | ScriptVariableConfig>;
  template_commands: Record<string, string>;
  template: string;
  label: string;
}
