import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const base = process.env.VITE_BASE_PATH ?? '/';

export default defineConfig({
  root: '.',
  base,
  plugins: [react(), wasm(), topLevelAwait()],
  resolve: {
    alias: {
      '@yodaos-pkg/aix': path.resolve(__dirname, '../dist'),
    },
  },
  server: {
    fs: {
      allow: ['..'],
    },
  },
});
