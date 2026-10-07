use dnd_m8_ruleset::{RulesetError, RulesetId};
use thiserror::Error;

use crate::{CharacterId, ConditionInstanceId, ResourceId};

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

    /// Hit-point state has current hit points above its maximum value.
    #[error("current hit points `{current}` exceed maximum hit points `{maximum}`")]
    InvalidHitPointState {
        /// Current hit points supplied to the model.
        current: u32,
        /// Maximum hit points supplied to the model.
        maximum: u32,
    },

    /// A resource indentifier was empty.
    #[error("resource identifier cannot be empty")]
    EmptyResourceId,

    /// Resource state has a current value above its maximum.
    #[error("resource `{id}` has current value `{current}` above maximum `{maximum}`")]
    InvalidResourcePool {
        /// Stable resource identifier.
        id: ResourceId,
        /// Current value supplied to the model.
        current: u32,
        /// Maximum value supplied to the model.
        maximum: u32,
    },

    /// The character contains multiple resources with the same identifier.
    #[error("resource `{id}` is already present")]
    DuplicateResource {
        /// Duplicate resource identifier.
        id: ResourceId,
    },

    /// The requested resource does not exist.
    #[error("resource `{id}` was not found")]
    ResourceNotFound {
        /// Missing resource identifier.
        id: ResourceId,
    },

    /// The resource does not have enough remaining uses.
    #[error("resourde `{id}` has only `{available}` available, but `{requested}` was requested")]
    InsufficientResource {
        /// Missing resource identifier.
        id: ResourceId,
        /// Ammount requested by the operation.
        requested: u32,
        /// Ammount available before the operation.
        available: u32,
    },

    /// A condition identifier was empty.
    #[error("condition identifier cannot be empty")]
    EmptyConditionId,

    /// A condition application with the same identifier already exists.
    #[error("condition application `{id}` is already active")]
    DuplicateConditionInstance {
        /// Duplicate runtime application identifier.
        id: ConditionInstanceId,
    },

    /// The requested condition application does not exist.
    #[error("condition application `{id}` was not found")]
    ConditionInstanceNotFound {
        /// Missing runtime application identifier.
        id: ConditionInstanceId,
    },
}
