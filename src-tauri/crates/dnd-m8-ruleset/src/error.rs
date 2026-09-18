//! Ruleset API error types.
use thiserror::Error;

use crate::RulesetId;

/// Result type alias used by ruleset API.
pub type RulesetResult<T> = std::result::Result<T, RulesetError>;

/// Ruleset API errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum RulesetError {
    /// Generic ruleset error.
    #[error("ruleset error: {msg}")]
    Generic {
        /// Generic ruleset error message.
        msg: String,
    },

    /// Ruleset family is either empty or whitespace.
    #[error("ruleset family cannot be empty")]
    EmptyFamily,

    /// Provided ruleset version does not use a valid semantic version syntax.
    #[error("invalid rileset version: {error}")]
    InvalidVersion {
        /// Version parsing error string.
        error: String,
    },

    /// Request to register a ruleset with an already registered ruleset id.
    #[error("ruleset `{id}` is already registered")]
    DuplicateRuleset {
        /// Requested ruleset identifier.
        id: RulesetId,
    },
}
