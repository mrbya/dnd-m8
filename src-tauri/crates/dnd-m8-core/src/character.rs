use std::fmt;

use dnd_m8_ruleset::{RulesetCharacterData, RulesetId};
use uuid::Uuid;

/// Unique identifier for characters managed by the app.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharacterId(Uuid);

impl CharacterId {
    /// Creates a new random character identifier.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Constructs a character identifier from an existing uuid.
    #[must_use]
    pub const fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    /// Returns the underlying uuid.
    #[must_use]
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for CharacterId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CharacterId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A character stored and managed by D&D Mate.
#[derive(Debug)]
pub struct Character {
    /// Managed character id.
    id: CharacterId,
    /// Id of ruleset the character is associated with.
    ruleset_id: RulesetId,
    /// Ruleset character data.
    ruleset_data: RulesetCharacterData,
}

impl Character {
    /// Creates a character associated with a particular ruleset.
    #[must_use]
    pub const fn new(
        id: CharacterId,
        ruleset_id: RulesetId,
        ruleset_data: RulesetCharacterData,
    ) -> Self {
        Self {
            id,
            ruleset_id,
            ruleset_data,
        }
    }

    /// Returns character id.
    #[must_use]
    pub const fn id(&self) -> CharacterId {
        self.id
    }

    /// Returns associated ruleset id.
    #[must_use]
    pub const fn ruleset_id(&self) -> &RulesetId {
        &self.ruleset_id
    }

    /// Returns ruleset character data.
    #[must_use]
    pub const fn ruleset_data(&self) -> &RulesetCharacterData {
        &self.ruleset_data
    }
}

#[cfg(test)]
mod tests {
    use crate::CharacterId;

    #[test]
    fn new_char_id_generates_random_uuid() {
        let id1 = CharacterId::new();
        let id2 = CharacterId::new();

        assert_ne!(id1, id2);
    }

    #[test]
    fn char_id_uuid_conversions() {
        let id1 = CharacterId::new();
        let id2 = CharacterId::from_uuid(*id1.as_uuid());

        assert_eq!(id1, id2);
    }
}
