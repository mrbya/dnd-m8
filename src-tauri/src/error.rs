use dnd_m8_core::CoreError;
use serde::Serialize;
use thiserror::Error;

/// Stable error category exposed through Tauri commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandErrorCode {
    /// A requested character does not exist.
    CharacterNotFound,

    /// Requested character already exists.
    DuplicateCharacter,

    /// A required ruleset is unavailable.
    RulesetNotFound,

    /// A ruleset could not process its character data.
    RulesetError,
}

/// Result returned by shell commands.
pub type CommandResult<T> = Result<T, CommandError>;

/// Serializable failure returned by a Tauri command.
#[derive(Debug, Error, Serialize)]
#[error("{message}")]
#[serde(rename_all = "camelCase")]
pub struct CommandError {
    /// Machine-readable error category.
    pub code: CommandErrorCode,

    /// Human-readable error message.
    pub message: String,
}

impl From<Box<CoreError>> for CommandError {
    fn from(value: Box<CoreError>) -> Self {
        let code = match *value {
            CoreError::CharacterNotFound { .. } => CommandErrorCode::CharacterNotFound,
            CoreError::DuplicateCharacter { .. } => CommandErrorCode::DuplicateCharacter,
            CoreError::RulesetNotFound { .. } => CommandErrorCode::RulesetNotFound,
            CoreError::Ruleset { .. } => CommandErrorCode::RulesetError,
        };

        Self {
            code,
            message: value.to_string(),
        }
    }
}
