/**
 * Read-only character projection returned by the Tauri backend.
 */
export interface CharacterSummaryDto {
  /** Stable character identifier. */
  readonly id: string;
  /** Character name. */
  readonly name: string;
  /** Total character level. */
  readonly level: number;
  /** User-facing class or multiclass description. */
  readonly className: string;
  /** Current hit points. */
  readonly currentHitPoints: number;
  /** Maximum hit points. */
  readonly maxHitPoints: number;
  /** Current armor class. */
  readonly armorClass: number;
  /** Current proficiency bonus. */
  readonly proficiencyBonus: number;
  /** Primary movement speed in feet. */
  readonly speed: number;
}

/**
 * Stable backend command error identifiers.
 */
export type CommandErrorCode =
  | "character_not_found"
  | "duplicate_character"
  | "ruleset_not_found"
  | "ruleset_error";

/**
 * Error returned across the Tauri invoke boundary.
 */
export interface CommandError {
  /** Machine-readable error category. */
  readonly code: CommandErrorCode;
  /** Human-readable failure explanation. */
  readonly message: string;
}
