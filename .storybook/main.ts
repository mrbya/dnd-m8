import type { StorybookConfig } from "@storybook/sveltekit";

const config = {
    stories: [
        // "../src/**/*.mdx",
        "../src/**/*.stories.@(js|ts|svelte)",
    ],
    addons: [
        "@storybook/addon-docs",
        "@storybook/addon-a11y",
        "@storybook/addon-vitest",
        "@storybook/addon-themes",
    ],
    framework: {
        name: "@storybook/sveltekit",
        options: {},
    },
    staticDirs: ["../static"],
} satisfies StorybookConfig;

export default config;
