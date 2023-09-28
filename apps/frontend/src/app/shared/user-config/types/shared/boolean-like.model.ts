import { z } from 'zod';

export const BooleanLikeSchema = z
  .union([z.boolean(), z.literal('true'), z.literal('false')])
  .transform(value => value === true || value === 'true');
