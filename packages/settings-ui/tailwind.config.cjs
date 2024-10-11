import tailwindConfig from '@glzr/components/tailwind.config.cjs'

/** @type {import('tailwindcss').Config} */
module.exports = {
  ...tailwindConfig,
  content: ["src/**/*.{ts,tsx}", "./node_modules/@glzr/components/dist/**/*.js"],
};
