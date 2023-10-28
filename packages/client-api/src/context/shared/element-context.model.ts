import { BaseElementConfig } from '~/user-config';
import { ElementType } from './element-type.model';

export interface ElementContext<T = unknown> {
  id: string;
  parent?: ElementContext;
  children: ElementContext[];
  rawConfig: unknown;
  parsedConfig: BaseElementConfig;
  type: ElementType;
  data: T;
}
