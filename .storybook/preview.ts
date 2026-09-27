import { withThemeByDataAttribute } from "@storybook/addon-themes";
import type { Preview, Renderer } from "@storybook/sveltekit";

import "../src/app.css";

const preview = {
    decorators: [
        withThemeByDataAttribute<Renderer>({
            themes: {
                Mocha: "dark",
                Latte: "light",
            },
            defaultTheme: "Mocha",
            attributeName: "data-theme",
        }),
    ],
    parameters: {
        layout: "centered",
        a11y: {
            test: "error",
        },
        controls: {
            matchers: {
                color: /(background|color)$/iu,
                date: /Date$/u,
            },
        },
    },
} satisfies Preview;

export default preview;
