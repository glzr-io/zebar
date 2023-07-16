import axios from 'axios';
import { createEffect, createResource, on } from 'solid-js';

import { memoize } from '../utils';
import { usePublicIp } from './use-public-ip.hook';
import { useLogger } from '../logging';

export enum WeatherStatus {
  CLEAR_DAY,
  CLEAR_NIGHT,
  CLOUDY_DAY,
  CLOUDY_NIGHT,
  OVERCAST,
  LIGHT_RAIN,
  HEAVY_RAIN,
  SNOW,
  THUNDER,
}

export interface OpenMeteoApiResponse {
  latitude: number;
  longitude: number;
  generationtime_ms: number;
  utc_offset_seconds: number;
  timezone: string;
  timezone_abbreviation: string;
  elevation: number;
  current_weather: {
    temperature: number;
    windspeed: number;
    winddirection: number;
    weathercode: number;
    is_day: number;
    time: string;
  };
  daily_units: {
    time: string;
    sunset: string;
    sunrise: string;
  };
  daily: {
    time: string[];
    sunset: string[];
    sunrise: string[];
  };
}

export const useWeather = memoize((latitude?: string, longitude?: string) => {
  const logger = useLogger('useWeather');
  const publicIp = usePublicIp();

  const [weather] = createResource(publicIp, async publicIp => {
    // Use OpenMeteo as provider for weather-related info.
    // Documentation: https://open-meteo.com/en/docs
    return axios
      .get<OpenMeteoApiResponse>('https://api.open-meteo.com/v1/forecast', {
        params: {
          latitude: latitude ?? publicIp.latitude,
          longitude: longitude ?? publicIp.longitude,
          temperature_unit: 'celsius',
          current_weather: true,
          daily: 'sunset,sunrise',
          timezone: 'auto',
        },
      })
      .then(({ data }) => {
        const currentWeather = data.current_weather;
        const isDaytime = currentWeather.is_day === 1;

        const weatherStatus = getWeatherStatus(
          currentWeather.weathercode,
          isDaytime,
        );

        return {
          isDaytime,
          weatherStatus,
          celsiusTemp: currentWeather.temperature,
          fahrenheitTemp: celsiusToFahrenheit(currentWeather.temperature),
          windSpeed: currentWeather.windspeed,
        };
      });
  });

  // Relevant documentation: https://open-meteo.com/en/docs#weathervariables
  function getWeatherStatus(code: number, isDaytime: boolean) {
    if (code === 0) {
      return isDaytime ? WeatherStatus.CLEAR_DAY : WeatherStatus.CLEAR_NIGHT;
    } else if (code === 1 || code === 2) {
      return isDaytime ? WeatherStatus.CLOUDY_DAY : WeatherStatus.CLOUDY_NIGHT;
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

  createEffect(
    on(weather, weather => logger.debug('Received weather data:', weather), {}),
  );

  return weather;
});
