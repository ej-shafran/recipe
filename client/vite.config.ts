/// <reference types="vitest" />


import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte({ hot: !process.env.VITEST })],
  server: {
    proxy: {
      "/api": "http://localhost:8000",
    },
  },
  test: {
    globals: true
  }
});
