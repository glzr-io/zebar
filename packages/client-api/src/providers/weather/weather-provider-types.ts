import type { Provider } from '../create-base-provider';

export interface WeatherProviderConfig {
  type: 'weather';

  /**
   * Latitude to retrieve weather for. If not provided, latitude is instead
   * estimated based on public IP.
   */
  latitude?: number;

  /**
   * Longitude to retrieve weather for. If not provided, longitude is instead
   * estimated based on public IP.
   */
  longitude?: number;

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

export type WeatherProvider = Provider<
  WeatherProviderConfig,
  WeatherOutput
>;

export interface WeatherOutput {
  isDaytime: boolean;
  status: WeatherStatus;
  celsiusTemp: number;
  fahrenheitTemp: number;
  windSpeed: number;
}

export enum WeatherStatus {
  CLEAR_DAY = 'clear_day',
  CLEAR_NIGHT = 'clear_night',
  CLOUDY_DAY = 'cloudy_day',
  CLOUDY_NIGHT = 'cloudy_night',
  LIGHT_RAIN_DAY = 'light_rain_day',
  LIGHT_RAIN_NIGHT = 'light_rain_night',
  HEAVY_RAIN_DAY = 'heavy_rain_day',
  HEAVY_RAIN_NIGHT = 'heavy_rain_night',
  SNOW_DAY = 'snow_day',
  SNOW_NIGHT = 'snow_night',
  THUNDER_DAY = 'thunder_day',
  THUNDER_NIGHT = 'thunder_night',
}
