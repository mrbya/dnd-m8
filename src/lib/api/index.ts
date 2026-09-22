/**
 * Single invoke boundary — all Tauri command calls live here.
 *
 * Views and stores must import from this file, never call `invoke()` directly.
 * Grouping all IPC calls in one place makes the contract between the frontend
 * and the Rust backend visible at a glance and simplifies mocking in tests.
 */
import { invoke } from "@tauri-apps/api/core";

import type { CharacterSummaryDto } from "$lib/types";

/**
 * Returns the character currently selected by the application shell.
 *
 * @returns The active character summary.
 * @throws A backend `CommandError` when the character cannot be summarized.
 */
export function getCharacter(): Promise<CharacterSummaryDto> {
  return invoke<CharacterSummaryDto>("get_character");
}
