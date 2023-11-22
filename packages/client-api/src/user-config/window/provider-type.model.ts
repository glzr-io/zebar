import { z } from 'zod';

export const ProviderTypeSchema = z.enum([
  'active_window',
  'battery',
  'cpu',
  'date',
  'glazewm',
  'host',
  'ip',
  'memory',
  'network',
  'self',
  'system_tray',
  'weather',
]);

export type ProviderType = z.infer<typeof ProviderTypeSchema>;
