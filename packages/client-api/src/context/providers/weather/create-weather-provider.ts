import axios from 'axios';
import { createEffect, on } from 'solid-js';
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

export interface WeatherVariables {
  isLoading: boolean;
  isDaytime: boolean;
  status: WeatherStatus;
  celsiusTemp: number;
  fahrenheitTemp: number;
  windSpeed: number;
}

export const createWeatherProvider = memoize(
  (options: WeatherProviderOptions = DEFAULT) => {
    const ipProvider = createIpProvider();

    const [weatherVariables, setWeatherVariables] =
      createStore<WeatherVariables>({
        isDaytime: true,
        status: WeatherStatus.CLEAR_DAY,
        celsiusTemp: 0,
        fahrenheitTemp: 0,
        windSpeed: 0,
        isLoading: true,
      });

    createEffect(
      on(
        () => !ipProvider.isLoading,
        () => refresh(),
      ),
    );

    async function refresh() {
      const {
        isLoading: isIpLoading,
        approxLatitude: latitude,
        approxLongitude: longitude,
      } = ipProvider;

      if (isIpLoading) {
        return;
      }

      // Use OpenMeteo as provider for weather-related info.
      // Documentation: https://open-meteo.com/en/docs
      const { data } = await axios.get<OpenMeteoApiResponse>(
        'https://api.open-meteo.com/v1/forecast',
        {
          params: {
            latitude: options.latitude ?? latitude,
            longitude: options.longitude ?? longitude,
            temperature_unit: 'celsius',
            current_weather: true,
            daily: 'sunset,sunrise',
            timezone: 'auto',
          },
        },
      );

      const currentWeather = data.current_weather;
      const isDaytime = currentWeather.is_day === 1;

      setWeatherVariables({
        isDaytime,
        status: getWeatherStatus(currentWeather.weathercode, isDaytime),
        celsiusTemp: currentWeather.temperature,
        fahrenheitTemp: celsiusToFahrenheit(currentWeather.temperature),
        windSpeed: currentWeather.windspeed,
        isLoading: false,
      });
    }

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
      get isDaytime() {
        return weatherVariables.isDaytime;
      },
      get status() {
        return weatherVariables.status;
      },
      get celsiusTemp() {
        return weatherVariables.celsiusTemp;
      },
      get fahrenheitTemp() {
        return weatherVariables.fahrenheitTemp;
      },
      get windSpeed() {
        return weatherVariables.windSpeed;
      },
      get isLoading() {
        return weatherVariables.isLoading;
      },
      refresh,
    };
  },
);
