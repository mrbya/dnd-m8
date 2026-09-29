export type ThemePreference = "system" | "light" | "dark";
export type ResolvedTheme = Exclude<ThemePreference, "system">;

export interface ThemeState {
    readonly preference: ThemePreference;
    readonly resolved: ResolvedTheme;
}

export interface ThemePlatform {
    readPreference(): string | null,
    writePreference(preference: ThemePreference): void,
    getSystemTheme(): ResolvedTheme,
    applyTheme(theme: ResolvedTheme): void,
    subscribeToSystemTheme(
        listener: (theme: ResolvedTheme) => void,
    ): () => void;
}

const storageKey = "dnd-m8.theme";
const darkModeQuery = "(prefers-color-scheme: dark)";

function isThemePreference(
    value: string | null,
): value is ThemePreference {
    return value === "system" || value === "light" || value === "dark";
}

const browserThemePlatform: ThemePlatform = {
    readPreference(): string | null {
        try {
            return window.localStorage.getItem(storageKey);
        } catch {
            return null;
        }
    },

    writePreference(preference: ThemePreference): void {
        try {
            window.localStorage.setItem(storageKey, preference);
        } catch {
            return;
        }
    },

    getSystemTheme(): ResolvedTheme {
        return window.matchMedia(darkModeQuery).matches ? "dark" : "light";
    },

    applyTheme(theme: ResolvedTheme) {
        document.documentElement.dataset.theme = theme;
    },

    subscribeToSystemTheme(
        listener: (theme: ResolvedTheme) => void,
    ): () => void {
        const query = window.matchMedia(darkModeQuery);

        const handleChange = (event: MediaQueryListEvent): void => {
            listener(event.matches ? "dark" : "light");
        };

        query.addEventListener("change", handleChange);

        return () => {
            query.removeEventListener("change", handleChange);
        };
    },
};

export class ThemeStore {
    readonly #platform: ThemePlatform;

    #stopListening: (() => void) | undefined;

    state = $state.raw<ThemeState>({
        preference: "system",
        resolved: "dark",
    });

    constructor(platform: ThemePlatform = browserThemePlatform) {
        this.#platform = platform;
    }

    start(): () => void {
        if (this.#stopListening !== undefined) {
            return () => { };
        }

        const storedPreference = this.#platform.readPreference();
        const preference = isThemePreference(storedPreference)
            ? storedPreference
            : "system";

        this.#resolveAndApply(preference);

        this.#stopListening = this.#platform.subscribeToSystemTheme(
            (theme) => {
                if (this.state.preference !== "system") {
                    return;
                }

                this.state = {
                    preference: "system",
                    resolved: theme,
                };

                this.#platform.applyTheme(theme);
            },
        );

        return () => {
            this.#stop();
        };
    }

    setPreference(preference: ThemePreference): void {
        this.#platform.writePreference(preference);
        this.#resolveAndApply(preference);
    }

    #resolveAndApply(preference: ThemePreference): void {
        const resolved =
            preference === "system"
                ? this.#platform.getSystemTheme()
                : preference;

        this.state = {
            preference,
            resolved,
        };

        this.#platform.applyTheme(resolved);
    }

    #stop(): void {
        this.#stopListening?.();
        this.#stopListening = undefined;
    }
}


export const themeStore = new ThemeStore();
