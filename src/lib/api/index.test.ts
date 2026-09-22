import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { afterEach, describe, expect, it } from "vitest";

import type { CharacterSummaryDto, CommandError } from "$lib/types";

import { getCharacter } from "./index";

const characterFixture: CharacterSummaryDto = {
  id: "10000000-0000-0000-0000-000000000001",
  name: "Mira Ashfall",
  level: 5,
  className: "Life Domain Cleric",
  currentHitPoints: 31,
  maxHitPoints: 38,
  armorClass: 18,
  proficiencyBonus: 3,
  speed: 30,
};

afterEach(() => {
  clearMocks();
});

describe("getCharacter", () => {
  it("invokes the get_character command", async () => {
    mockIPC((command) => {
      expect(command).toBe("get_character");

      return characterFixture;
    });

    await expect(getCharacter()).resolves.toEqual(characterFixture);
  });

  it("preserves backend command errors", async () => {
    const backendError: CommandError = {
      code: "ruleset_error",
      message: "character data could not be summarized",
    };

    mockIPC(() => {
      throw backendError;
    });

    await expect(getCharacter()).rejects.toEqual(backendError);
  });
});
