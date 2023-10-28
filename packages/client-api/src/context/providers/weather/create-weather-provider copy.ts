import axios from 'axios';
import { createEffect, createResource, on } from 'solid-js';
import { createStore } from 'solid-js/store';

import { memoize } from '~/utils';
import {
  WeatherProviderOptions,
  WeatherProviderOptionsSchema,
} from '~/user-config';
import { createIpProvider } from '../ip/create-ip-provider';
import { WeatherStatus } from './weather-status.enum';
import { OpenMeteoApiResponse } from './open-meteo-api-response.model';

const DEFAULT = WeatherProviderOptionsSchema.parse({});

export const createWeatherProvider = memoize(
  (options: WeatherProviderOptions = DEFAULT) => {
    const ipProvider = createIpProvider();

    const [weatherData, { refetch: refresh }] = createResource(
      ipProvider,
      async ipProvider => {
        // Use OpenMeteo as provider for weather-related info.
        // Documentation: https://open-meteo.com/en/docs
        const { data } = await axios.get<OpenMeteoApiResponse>(
          'https://api.open-meteo.com/v1/forecast',
          {
            params: {
              latitude: options.latitude ?? ipProvider.latitude,
              longitude: options.longitude ?? ipProvider.longitude,
              temperature_unit: 'celsius',
              current_weather: true,
              daily: 'sunset,sunrise',
              timezone: 'auto',
            },
          },
        );

        const currentWeather = data.current_weather;
        const isDaytime = currentWeather.is_day === 1;

        return {
          is_day_time: isDaytime,
          status: getWeatherStatus(currentWeather.weathercode, isDaytime),
          celsius_temp: currentWeather.temperature,
          fahrenheit_temp: celsiusToFahrenheit(currentWeather.temperature),
          wind_speed: currentWeather.windspeed,
          is_loading: false,
          is_refreshing: false,
        };
      },
    );

    // Relevant documentation: https://open-meteo.com/en/docs#weathervariables
    function getWeatherStatus(code: number, isDaytime: boolean) {
      if (code === 0) {
        return isDaytime ? WeatherStatus.CLEAR_DAY : WeatherStatus.CLEAR_NIGHT;
      } else if (code === 1 || code === 2) {
        return isDaytime
          ? WeatherStatus.CLOUDY_DAY
          : WeatherStatus.CLOUDY_NIGHT;
      } else if (code >= 3) {
        return WeatherStatus.OVERCAST;
      } else if (code >= 51) {
        return WeatherStatus.LIGHT_RAIN;
      } else if (code >= 63) {
        return WeatherStatus.HEAVY_RAIN;
      } else if (code >= 71) {
        return WeatherStatus.SNOW;
      } else if (code >= 80) {
        return WeatherStatus.HEAVY_RAIN;
      } else if (code >= 85) {
        return WeatherStatus.SNOW;
      } else if (code >= 95) {
        return WeatherStatus.SNOW;
      }
    }

    function celsiusToFahrenheit(celsiusTemp: number) {
      return (celsiusTemp * 9) / 5 + 32;
    }

    return {
      get is_day_time() {
        return weatherData()?.is_day_time ?? true;
      },
      get status() {
        return weatherData()?.status ?? WeatherStatus.CLEAR_DAY;
      },
      get celsius_temp() {
        return weatherData()?.celsius_temp ?? 0;
      },
      get fahrenheit_temp() {
        return weatherData()?.fahrenheit_temp ?? 0;
      },
      get wind_speed() {
        return weatherData()?.wind_speed ?? 0;
      },
      get is_loading() {
        return weatherData()?.is_loading ?? true;
      },
      get is_refreshing() {
        return weatherData()?.is_refreshing ?? false;
      },
      refresh,
    };
  },
);
