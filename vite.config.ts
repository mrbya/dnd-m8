/// <reference types="vitest/config" />
/// <reference types="vitest" />
/// <reference types="vite/client" />
import { defineConfig } from "vitest/config";
import { playwright } from "@vitest/browser-playwright";
import { sveltekit } from "@sveltejs/kit/vite";
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { storybookTest } from '@storybook/addon-vitest/vitest-plugin';
const dirname = typeof __dirname !== 'undefined' ? __dirname : path.dirname(fileURLToPath(import.meta.url));

// More info at: https://storybook.js.org/docs/next/writing-tests/integrations/vitest-addon
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig({
    plugins: [sveltekit()],
    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    //
    // 1. prevent Vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
        port: 1420,
        strictPort: true,
        host: host || "127.0.0.1",
        hmr: host ? {
            protocol: "ws",
            host,
            port: 1421
        } : undefined,
        watch: {
            // 3. tell Vite to ignore watching `src-tauri`
            ignored: ["**/src-tauri/**"]
        }
    },
    test: {
        expect: {
            requireAssertions: true
        },
        projects: [{
            extends: true,
            test: {
                name: "client",
                browser: {
                    enabled: true,
                    provider: playwright(),
                    instances: [{
                        browser: "chromium",
                        headless: true
                    }]
                },
                include: ["src/**/*.svelte.{test,spec}.{js,ts}", "src/**/*.browser.{test,spec}.{js,ts}"],
                exclude: ["src/lib/server/**"]
            }
        }, {
            extends: true,
            test: {
                name: "server",
                environment: "node",
                include: ["src/**/*.{test,spec}.{js,ts}"],
                exclude: ["src/**/*.svelte.{test,spec}.{js,ts}", "src/**/*.browser.{test,spec}.{js,ts}", "src/lib/server/**"]
            }
        }, {
            extends: true,
            plugins: [
                // The plugin will run tests for the stories defined in your Storybook config
                // See options at: https://storybook.js.org/docs/next/writing-tests/integrations/vitest-addon#storybooktest
                storybookTest({
                    configDir: path.join(import.meta.dirname, ".storybook"),
                })],
            test: {
                name: 'storybook',
                browser: {
                    enabled: true,
                    headless: true,
                    provider: playwright({}),
                    instances: [{
                        browser: 'chromium'
                    }]
                }
            }
        }]
    }
});
