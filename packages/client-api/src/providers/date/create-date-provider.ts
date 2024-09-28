import { DateTime } from 'luxon';
import { z } from 'zod';

import { createBaseProvider } from '../create-base-provider';
import type {
  DateProvider,
  DateProviderConfig,
} from './date-provider-types';

const dateProviderConfigSchema = z.object({
  type: z.literal('date'),
  refreshInterval: z.coerce.number().default(1000),
  timezone: z.string().default('local'),
  locale: z.string().optional(),
  formatting: z.string().default('EEE	d MMM t'),
});

export function createDateProvider(
  config: DateProviderConfig,
): DateProvider {
  const mergedConfig = dateProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    queue.output(getDateValue());

    const interval = setInterval(
      () => queue.output(getDateValue()),
      mergedConfig.refreshInterval,
    );

    function getDateValue() {
      const dateTime = DateTime.now().setZone(mergedConfig.timezone);

      return {
        new: dateTime.toJSDate(),
        now: dateTime.toMillis(),
        iso: dateTime.toISO()!,
        formatted: dateTime.toFormat(mergedConfig.formatting, {
          locale: mergedConfig.locale,
        }),
      };
    }

    return () => {
      clearInterval(interval);
    };
  });
}
