import { JSXElement } from 'solid-js';

export interface TemplateBindings {
  strings?: Record<string, string | boolean | number>;
  functions?: Record<string, (...args: unknown[]) => unknown>;
  components?: Record<string, () => JSXElement>;
}
