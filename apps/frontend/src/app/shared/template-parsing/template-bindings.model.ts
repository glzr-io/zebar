import { JSXElement } from 'solid-js';

type SupportedPrimitive = string | boolean | number;

type TemplateVariable =
  | SupportedPrimitive
  | SupportedPrimitive[]
  | { [name: string]: TemplateVariable }
  | { [name: string]: TemplateVariable }[];

export interface TemplateBindings {
  variables?: Record<string, TemplateVariable>;
  slots?: Record<string, string>;
  functions?: Record<string, (...args: unknown[]) => unknown>;
  components?: Record<string, () => JSXElement>;
}
