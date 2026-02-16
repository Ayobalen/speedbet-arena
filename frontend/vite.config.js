import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import { fileURLToPath } from 'url';
import { dirname, resolve } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      // Use mock in development, real package in production (loaded via CDN)
      '@linera/client': resolve(__dirname, 'src/stubs/linera-client.js'),
    },
  },
  build: {
    rollupOptions: {
      external: ['@linera/client'],
    },
  },
  optimizeDeps: {
    exclude: ['@linera/client'],
  },
  server: {
    host: '0.0.0.0',
    port: 8080,
  },
});
