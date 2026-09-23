import { getCharacter } from "$lib/api";
import type {
  CharacterSummaryDto,
  CommandError,
  CommandErrorCode,
} from "$lib/types";

export type CharacterLoader = () => Promise<CharacterSummaryDto>;

export interface CharacterLoadError {
  readonly code: CommandErrorCode | "unexpected";
  readonly message: string;
}

export type CharacterState =
  | { readonly status: "idle" }
  | { readonly status: "loading" }
  | {
      readonly status: "ready";
      readonly character: CharacterSummaryDto;
    }
  | {
      readonly status: "error";
      readonly error: CharacterLoadError;
    };

const commandErrorCodes = new Set<CommandErrorCode>([
  "character_not_found",
  "duplicate_character",
  "ruleset_not_found",
  "ruleset_error",
]);

function isCommandError(error: unknown): error is CommandError {
  if (typeof error !== "object" || error === null) {
    return false;
  }

  const candidate = error as Record<string, unknown>;

  return (
    typeof candidate.code === "string" &&
    commandErrorCodes.has(candidate.code as CommandErrorCode) &&
    typeof candidate.message === "string"
  );
}

function normalizeError(error: unknown): CharacterLoadError {
  if (isCommandError(error)) {
    return error;
  }

  return {
    code: "unexpected",
    message:
      error instanceof Error
        ? error.message
        : "The character could not be loaded.",
  };
}

export class CharacterStore {
  readonly #loadCharacter: CharacterLoader;

  state = $state.raw<CharacterState>({ status: "idle" });

  constructor(loadCharacter: CharacterLoader = getCharacter) {
    this.#loadCharacter = loadCharacter;
  }

  async load(): Promise<void> {
    if (this.state.status === "loading") {
      return;
    }

    this.state = { status: "loading" };

    try {
      const character = await this.#loadCharacter();

      this.state = {
        status: "ready",
        character,
      };
    } catch (error: unknown) {
      this.state = {
        status: "error",
        error: normalizeError(error),
      };
    }
  }
}
