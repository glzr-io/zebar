import { z } from 'zod';

export const ProviderTypeSchema = z.enum([
  'active_window',
  'battery',
  'cpu',
  'custom',
  'date_time',
  'glazewm',
  'ip',
  'memory',
  'network',
  'system_tray',
  'weather',
]);

export type ProviderType = z.infer<typeof ProviderTypeSchema>;
