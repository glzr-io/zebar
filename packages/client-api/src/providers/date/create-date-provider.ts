import { DateTime } from 'luxon';
import { z } from 'zod';

import { createBaseProvider } from '../create-base-provider';

export interface DateProviderConfig {
  type: 'date';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;

  /**
   * Either a UTC offset (eg. `UTC+8`) or an IANA timezone (eg.
   * `America/New_York`). Affects the output of `toFormat()`.
   *
   * A full list of available IANA timezones can be found [here](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones#List).
   */
  timezone?: string;

  /**
   * An ISO-639-1 locale, which is either a 2-letter language code (eg. `en`) or
   * 4-letter language + country code (eg. `en-gb`). Affects the output of
   * `toFormat()`.
   *
   * A full list of ISO-639-1 locales can be found [here](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes#Table).
   */
  locale?: string;

  /**
   * Formatting of the current date into a custom string format.
   *
   * Refer to [table of tokens](https://moment.github.io/luxon/#/formatting?id=table-of-tokens)
   * for available date/time tokens.
   *
   * @example
   * "yyyy LLL dd" -> "2023 Feb 13"
   * "HH 'hours and' mm 'minutes'" -> "20 hours and 55 minutes"
   */
  formatting?: string;
}

const DateProviderConfigSchema = z.object({
  type: z.literal('date'),
  refreshInterval: z.coerce.number().default(1000),
  timezone: z.string().optional(),
  locale: z.string().optional(),
});

export interface DateOutput {
  /**
   * Current date/time as a JavaScript `Date` object. Uses `new Date()` under
   * the hood.
   **/
  new: Date;

  /**
   * Current date/time as milliseconds since epoch. Uses `Date.now()` under the
   * hood.
   **/
  now: number;

  /**
   * Current date/time as an ISO-8601 string (eg.
   * `2017-04-22T20:47:05.335-04:00`). Uses `date.toISOString()` under the hood.
   **/
  iso: string;
}

// TODO: Implement `createBaseProvider` for all providers.
// TODO: Remove `createProviderListener`. Instead move listen function to `desktop-events.ts`.
// TODO: Organize provider-related types.
// TODO: Remove `toFormat` on date provider. Instead add `formatting` to the config.
export async function createDateProvider(
  config: DateProviderConfig,
): Promise<DateOutput> {
  const mergedConfig = DateProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, queue => {
    queue.value(getDateValue());

    const interval = setInterval(
      () => queue.value(getDateValue()),
      mergedConfig.refreshInterval,
    );

    function getDateValue() {
      const date = new Date();

      return {
        new: date,
        now: date.getTime(),
        iso: date.toISOString(),
        formatted: 'todo',
      };
    }

    return () => {
      clearInterval(interval);
    };
  });
}
