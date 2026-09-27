import type { CharacterSummaryDto, CommandError } from "$lib/types";
import { describe, expect, it, vi } from "vitest";
import { CharacterStore, type CharacterLoader } from "./caracter.svelte";

const characterFixture: CharacterSummaryDto = {
  id: "10000000-0000-0000-0000-000000000001",
  name: "Bruce Lee",
  level: 5,
  className: "Monk",
  currentHitPoints: 31,
  maxHitPoints: 38,
  armorClass: 18,
  proficiencyBonus: 3,
  speed: 30,
};

describe("CharacterStore", () => {
  it("loads a character", async () => {
    const loader = vi.fn<CharacterLoader>().mockResolvedValue(characterFixture);

    const store = new CharacterStore(loader);

    const loading = store.load();

    expect(store.state).toEqual({ status: "loading" });

    await loading;

    expect(loader).toHaveBeenCalledOnce();
    expect(store.state).toEqual({
      status: "ready",
      character: characterFixture,
    });
  });

  it("preserves structured command errors", async () => {
    const backendError: CommandError = {
      code: "ruleset_error",
      message: "character data could not be summarized",
    };

    const loader = vi.fn<CharacterLoader>().mockRejectedValue(backendError);

    const store = new CharacterStore(loader);

    await store.load();

    expect(store.state).toEqual({
      status: "error",
      error: backendError,
    });
  });

  it("normalizes unexpected errors", async () => {
    const loader = vi
      .fn<CharacterLoader>()
      .mockRejectedValue(new Error("IPC transport failed"));

    const store = new CharacterStore(loader);

    await store.load();

    expect(store.state).toEqual({
      status: "error",
      error: {
        code: "unexpected",
        message: "IPC transport failed",
      },
    });
  });

  it("does not start concurrent loads", async () => {
    let resolvedCharacter!: (character: CharacterSummaryDto) => void;

    const pendingCharacter = new Promise<CharacterSummaryDto>((resolve) => {
      resolvedCharacter = resolve;
    });

    const loader = vi.fn<CharacterLoader>(() => pendingCharacter);
    const store = new CharacterStore(loader);

    const firstLoad = store.load();
    await store.load();

    expect(loader).toHaveBeenCalledOnce();

    resolvedCharacter(characterFixture);
    await firstLoad;

    expect(store.state.status).toBe("ready");
  });
});
