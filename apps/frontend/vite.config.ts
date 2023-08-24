import path from 'path';
import { defineConfig } from 'vite';

export default defineConfig({
  // Prevent vite from obscuring Rust errors.
  clearScreen: false,
  // Tauri expects a fixed port. Fail if that port is not available.
  server: {
    port: 4200,
    strictPort: true,
  },
  // Allow use of `TAURI_DEBUG` and other env variables.
  // Ref: https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    // Tauri supports ES2021.
    target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
    // Don't minify for debug builds.
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // Produce sourcemaps for debug builds.
    sourcemap: !!process.env.TAURI_DEBUG,
  },
  resolve: {
    alias: {
      '~': path.resolve(__dirname, './src/app'),
    },
  },
  css: {
    modules: {
      localsConvention: 'dashes',
    },
  },
});
