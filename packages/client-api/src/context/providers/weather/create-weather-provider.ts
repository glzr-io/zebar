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

export const createWeatherProvider = memoize(
  (options: WeatherProviderOptions = DEFAULT) => {
    const ipProvider = createIpProvider();

    const [weatherVariables, setWeatherVariables] = createStore({
      is_day_time: true,
      status: WeatherStatus.CLEAR_DAY,
      celsius_temp: 0,
      fahrenheit_temp: 0,
      wind_speed: 0,
      is_loading: true,
      is_refreshing: false,
    });

    createEffect(
      on(
        () => !ipProvider.variables.is_loading,
        () => refresh(),
      ),
    );

    async function refresh() {
      const {
        is_loading: isIpLoading,
        latitude,
        longitude,
      } = ipProvider.variables;

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
        is_day_time: isDaytime,
        status: getWeatherStatus(currentWeather.weathercode, isDaytime),
        celsius_temp: currentWeather.temperature,
        fahrenheit_temp: celsiusToFahrenheit(currentWeather.temperature),
        wind_speed: currentWeather.windspeed,
        is_loading: false,
        is_refreshing: false,
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
      variables: weatherVariables,
      commands: {
        refresh,
      },
    };
  },
);
