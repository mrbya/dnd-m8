use dnd_m8_ruleset::{RulesetError, RulesetId};
use thiserror::Error;

use crate::CharacterId;

/// Result type alias used by core API.
pub type CoreResult<T> = std::result::Result<T, Box<CoreError>>;

/// D&D Mate Core API errors.
#[derive(Debug, Error)]
pub enum CoreError {
    /// The reqyested character does not exist.
    #[error("character `{id}` was not found")]
    CharacterNotFound {
        /// Id of the character.
        id: CharacterId,
    },

    /// A character with the same id is already registered.
    #[error("character `{id}` is already registered")]
    DuplicateCharacter {
        /// Id of the duplicate character.
        id: CharacterId,
    },

    /// The character references a ruleset unavailable to the application.
    #[error("ruleset `{ruleset_id}` required by character `{character_id}` was not found")]
    RulesetNotFound {
        /// Id of the character in question.
        character_id: CharacterId,
        /// Id of the characters associated ruleset.
        ruleset_id: RulesetId,
    },

    /// Internal ruleset error.
    #[error("character `{character_id}` encountered an error according to ruleset `{ruleset_id}`: {source}")]
    Ruleset {
        /// Id of the character in question.
        character_id: CharacterId,
        /// Id of the characters associated ruleset.
        ruleset_id: RulesetId,

        /// Underlying ruleset error.
        #[source]
        source: RulesetError,
    },
}
