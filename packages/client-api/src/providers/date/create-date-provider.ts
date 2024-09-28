import { DateTime } from 'luxon';
import { z } from 'zod';

import {
  createBaseProvider,
  type Provider,
} from '../create-base-provider';

export interface DateProviderConfig {
  type: 'date';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;

  /**
   * Either a UTC offset (eg. `UTC+8`) or an IANA timezone (eg.
   * `America/New_York`). Affects the output of {@link DateOutput.formatted}.
   *
   * A full list of available IANA timezones can be found [here](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones#List).
   */
  timezone?: string;

  /**
   * An ISO-639-1 locale, which is either a 2-letter language code
   * (eg. `en`) or a 4-letter language + country code (eg. `en-gb`).
   * Affects the output of {@link DateOutput.formatted}.
   *
   * A full list of ISO-639-1 locales can be found [here](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes#Table).
   */
  locale?: string;

  /**
   * Formatting of the current date into a custom string format. Affects
   * the output of {@link DateOutput.formatted}.
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

const dateProviderConfigSchema = z.object({
  type: z.literal('date'),
  refreshInterval: z.coerce.number().default(1000),
  timezone: z.string().default('local'),
  locale: z.string().optional(),
  formatting: z.string().default('EEE	d MMM t'),
});

export type DateProvider = Provider<DateProviderConfig, DateOutput>;

export interface DateOutput {
  /**
   * Current date/time as a formatted string.
   */
  formatted: string;

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

export function createDateProvider(
  config: DateProviderConfig,
): DateProvider {
  const mergedConfig = dateProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, queue => {
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
