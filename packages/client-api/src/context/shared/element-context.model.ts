import { BaseElementConfig } from '~/user-config';
import { ElementType } from './element-type.model';

export interface ElementContext<T = unknown> {
  id: string;
  rawConfig: unknown;
  parsedConfig: BaseElementConfig;
  type: ElementType;
  data: T;
  childIds: string[];
  getChild: (id: string) => ElementContext | null;
}
