import type {
  GlobalConfig,
  GroupConfig,
  TemplateConfig,
  WindowConfig,
} from '~/user-config';
import { ElementType } from './element-type.model';

export type ElementConfig = WindowConfig | GroupConfig | TemplateConfig;

interface BaseElementContext<C extends ElementConfig, P = {}> {
  /**
   * ID of this element.
   */
  id: string;

  /**
   * Type of this element.
   */
  type: ElementType;

  /**
   * Unparsed config for this element.
   */
  rawConfig: unknown;

  /**
   * Parsed config for this element.
   */
  parsedConfig: C;

  /**
   * Global user config.
   */
  globalConfig: GlobalConfig;

  /**
   * Args used to open the window.
   */
  args: Record<string, string>;

  /**
   * Environment variables when window was opened.
   */
  env: Record<string, string>;

  /**
   * Map of this element's providers and their variables.
   */
  providers: P;

  // TODO: Consider adding `scripts` record where keys are script paths.

  /**
   * Initializes a child group or template element.
   * @internal
   */
  initChildElement: (
    id: string,
  ) => Promise<GroupContext | TemplateContext | null>;
}

export type WindowContext<P = {}> = BaseElementContext<WindowConfig, P>;
export type GroupContext<P = {}> = BaseElementContext<GroupConfig, P>;
export type TemplateContext<P = {}> = BaseElementContext<
  TemplateConfig,
  P
>;

export type ElementContext =
  | WindowContext
  | GroupContext
  | TemplateContext;
