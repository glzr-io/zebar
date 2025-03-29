import path from 'path';
import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';

export default defineConfig({
  plugins: [solidPlugin()],
  server: {
    port: 4200,
  },
  build: { target: 'esnext' },
  base: './',
  optimizeDeps: {
    include: ['solid-markdown > micromark', 'solid-markdown > unified'],
  },
  resolve: {
    alias: {
      '~': path.resolve(__dirname, './src'),
    },
  },
});
