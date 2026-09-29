import { describe, expect, it, vi } from "vitest";

import {
  ThemeStore,
  type ResolvedTheme,
  type ThemePlatform,
} from "./theme.svelte";

function createPlatform(
  storedPreference: string | null = null,
  systemTheme: ResolvedTheme = "dark",
) {
  let systemThemeListener: ((theme: ResolvedTheme) => void) | undefined;

  const unsubscribe = vi.fn();

  const readPreference = vi.fn<ThemePlatform["readPreference"]>(
    () => storedPreference,
  );

  const writePreference = vi.fn<ThemePlatform["writePreference"]>();

  const getSystemTheme = vi.fn<ThemePlatform["getSystemTheme"]>(
    () => systemTheme,
  );

  const applyTheme = vi.fn<ThemePlatform["applyTheme"]>();

  const subscribeToSystemTheme = vi.fn<ThemePlatform["subscribeToSystemTheme"]>(
    (listener) => {
      systemThemeListener = listener;

      return unsubscribe;
    },
  );

  const platform: ThemePlatform = {
    readPreference,
    writePreference,
    getSystemTheme,
    applyTheme,
    subscribeToSystemTheme,
  };

  return {
    platform,
    readPreference,
    writePreference,
    getSystemTheme,
    applyTheme,
    subscribeToSystemTheme,
    unsubscribe,

    emitSystemTheme(theme: ResolvedTheme): void {
      systemThemeListener?.(theme);
    },
  };
}

describe("ThemeStore", () => {
  it("uses the current system theme by default", () => {
    const fixture = createPlatform(null, "light");
    const store = new ThemeStore(fixture.platform);

    const stop = store.start();

    expect(store.state).toEqual({
      preference: "system",
      resolved: "light",
    });

    expect(fixture.readPreference).toHaveBeenCalledOnce();
    expect(fixture.getSystemTheme).toHaveBeenCalledOnce();
    expect(fixture.applyTheme).toHaveBeenCalledWith("light");

    stop();

    expect(fixture.unsubscribe).toHaveBeenCalledOnce();
  });

  it("restores a stored explicit preference", () => {
    const fixture = createPlatform("dark", "light");
    const store = new ThemeStore(fixture.platform);

    store.start();

    expect(store.state).toEqual({
      preference: "dark",
      resolved: "dark",
    });

    expect(fixture.getSystemTheme).not.toHaveBeenCalled();
    expect(fixture.applyTheme).toHaveBeenCalledWith("dark");
  });

  it("ignores invalid stored preferences", () => {
    const fixture = createPlatform("sepia", "light");
    const store = new ThemeStore(fixture.platform);

    store.start();

    expect(store.state).toEqual({
      preference: "system",
      resolved: "light",
    });

    expect(fixture.getSystemTheme).toHaveBeenCalledOnce();
  });

  it("persists and applies an explicit preference", () => {
    const fixture = createPlatform(null, "dark");
    const store = new ThemeStore(fixture.platform);

    store.start();
    store.setPreference("light");

    expect(fixture.writePreference).toHaveBeenCalledWith("light");
    expect(fixture.applyTheme).toHaveBeenLastCalledWith("light");

    expect(store.state).toEqual({
      preference: "light",
      resolved: "light",
    });
  });

  it("resolves the system theme when selecting system", () => {
    const fixture = createPlatform("dark", "light");
    const store = new ThemeStore(fixture.platform);

    store.start();
    store.setPreference("system");

    expect(fixture.writePreference).toHaveBeenCalledWith("system");
    expect(fixture.getSystemTheme).toHaveBeenCalledOnce();
    expect(fixture.applyTheme).toHaveBeenLastCalledWith("light");

    expect(store.state).toEqual({
      preference: "system",
      resolved: "light",
    });
  });

  it("reacts to system changes only for the system preference", () => {
    const fixture = createPlatform(null, "dark");
    const store = new ThemeStore(fixture.platform);

    store.start();

    fixture.emitSystemTheme("light");

    expect(store.state).toEqual({
      preference: "system",
      resolved: "light",
    });

    store.setPreference("dark");
    fixture.emitSystemTheme("light");

    expect(store.state).toEqual({
      preference: "dark",
      resolved: "dark",
    });

    expect(fixture.applyTheme).toHaveBeenLastCalledWith("dark");
  });

  it("does not create multiple system-theme subscriptions", () => {
    const fixture = createPlatform();
    const store = new ThemeStore(fixture.platform);

    const firstStop = store.start();
    const secondStop = store.start();

    expect(fixture.subscribeToSystemTheme).toHaveBeenCalledOnce();

    secondStop();

    expect(fixture.unsubscribe).not.toHaveBeenCalled();

    firstStop();

    expect(fixture.unsubscribe).toHaveBeenCalledOnce();
  });
});
