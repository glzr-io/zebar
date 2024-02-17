import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const DateProviderConfigSchema = z.object({
  type: z.literal(ProviderType.DATE),

  refresh_interval: z.coerce.number().default(1000),

  /**
   * Either a UTC offset (eg. `UTC+8`) or an IANA timezone (eg.
   * `America/New_York`). Affects the output of `toFormat()`.
   *
   * A full list of available IANA timezones can be found [here](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones#List).
   */
  timezone: z.string().optional(),

  /**
   * An ISO-639-1 locale, which is either a 2-letter language code (eg. `en`) or
   * 4-letter language + country code (eg. `en-gb`). Affects the output of
   * `toFormat()`.
   *
   * A full list of ISO-639-1 locales can be found [here](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes#Table).
   */
  locale: z.string().optional(),
});

export type DateProviderConfig = z.infer<typeof DateProviderConfigSchema>;
