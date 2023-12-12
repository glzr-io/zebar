import { BaseElementConfig } from '~/user-config';
import { ElementType } from './element-type.model';

export interface ElementContext<T = unknown> {
  id: string;
  rawConfig: unknown;
  parsedConfig: BaseElementConfig;
  type: ElementType;
  variables: T;
  childIds: string[];
  initChild: (id: string) => ElementContext | null;
}
