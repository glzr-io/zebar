import axios from 'axios';
import { createResource } from 'solid-js';

import { memoize } from '../utils';
import { usePublicIp } from './use-public-ip.hook';

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

// TODO: Remove `memoize` and instead pass latitude and longitde as args.
export const useWeather = memoize(() => {
  const publicIp = usePublicIp();

  const [weather] = createResource(publicIp, async publicIp => {
    const weather = await axios.get(
      `https://wttr.in/${publicIp.city}+${publicIp.country}?format=j1`,
    );

    // Use OpenMeteo as provider for weather-related info.
    // Documentation: https://open-meteo.com/en/docs
    const res = await axios.get('https://api.open-meteo.com/v1/forecast', {
      params: {
        latitude: publicIp.latitude,
        longitude: publicIp.longitude,
        temperature_unit: 'celsius',
        current_weather: true,
        daily: 'sunset, sunrise',
        timezone: 'auto',
      },
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

  return weather;
});
