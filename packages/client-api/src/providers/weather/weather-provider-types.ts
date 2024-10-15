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

export type WeatherStatus =
  | 'clear_day'
  | 'clear_night'
  | 'cloudy_day'
  | 'cloudy_night'
  | 'light_rain_day'
  | 'light_rain_night'
  | 'heavy_rain_day'
  | 'heavy_rain_night'
  | 'snow_day'
  | 'snow_night'
  | 'thunder_day'
  | 'thunder_night';
