import { BaseElementConfig } from '~/user-config';

export interface ElementContext<T = unknown> {
  id: string;
  parent?: ElementContext;
  children: ElementContext[];
  rawConfig: unknown;
  parsedConfig: BaseElementConfig;
  data: T;
}
