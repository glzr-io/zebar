import { z } from 'zod';

import { ComponentGroupConfigSchema } from './component-group-config.model';
import { delimitedKey } from '../../delimited-key';
import { Prettify } from '~/shared/utils';
import { ElementSchema } from '../shared/element.model';

export const BarConfigSchema = ElementSchema.extend({
  group: ComponentGroupConfigSchema.optional(),
}).and(z.record(delimitedKey('group'), ComponentGroupConfigSchema).optional());

export type BarConfig = Prettify<z.infer<typeof BarConfigSchema>>;
