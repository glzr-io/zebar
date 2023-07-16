import { JSXElement } from 'solid-js';

export interface TemplateBindings {
  strings?: Record<string, string | boolean | number>;
  slots?: Record<string, string>;
  functions?: Record<string, (...args: unknown[]) => unknown>;
  components?: Record<string, () => JSXElement>;
}
