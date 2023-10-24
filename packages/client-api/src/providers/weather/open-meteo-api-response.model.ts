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
