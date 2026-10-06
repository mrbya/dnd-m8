use std::fmt;

use dnd_m8_ruleset::{RulesetCharacterData, RulesetId};
use uuid::Uuid;

/// Hit-point runtime state and transitions.
mod hitpoints;
/// Generic limited-resource runtime state.
mod resources;
/// Ruleset-independent character runtime state.
mod state;
/// Combined character read model.
mod summary;

// Re-exports.
pub use hitpoints::{DamageOutcome, HealingOutcome, HitPointState};
pub use resources::{
    ResourceId, ResourcePool, ResourcePools, ResourceRestoreOutcome, ResourceSpendOutcome,
};
pub use state::CharacterState;
pub use summary::CharacterSummary;

use crate::CoreResult;

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
    /// Ruleset-independent runtime state.
    state: CharacterState,
}

impl Character {
    /// Creates a character associated with a particular ruleset.
    #[must_use]
    pub const fn new(
        id: CharacterId,
        ruleset_id: RulesetId,
        ruleset_data: RulesetCharacterData,
        state: CharacterState,
    ) -> Self {
        Self {
            id,
            ruleset_id,
            ruleset_data,
            state,
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

    /// Returns ruleset-independent runtime state.
    #[must_use]
    pub const fn state(&self) -> &CharacterState {
        &self.state
    }

    /// Applies damage to the character's hit points.
    #[must_use]
    pub fn take_damage(&mut self, amount: u32) -> DamageOutcome {
        self.state.hit_points_mut().take_damage(amount)
    }

    /// Restores the character's ordinary hit points.
    #[must_use]
    pub fn heal(&mut self, amount: u32) -> HealingOutcome {
        self.state.hit_points_mut().heal(amount)
    }

    /// Replaces temporary hit points and returns their previous amount.
    #[must_use]
    pub const fn replace_temporary_hit_points(&mut self, amount: u32) -> u32 {
        self.state.hit_points_mut().replace_temporary(amount)
    }

    /// Spends one of this character's resources.
    ///
    /// # Errors
    ///
    /// Returns an error when the resource is missing or insufficient.
    pub fn spend_resource(
        &mut self,
        id: &ResourceId,
        amount: u32,
    ) -> CoreResult<ResourceSpendOutcome> {
        self.state.resources_mut().spend(id, amount)
    }

    /// Restores one of this character's resources.
    ///
    /// # Errors
    ///
    /// Returns an error when the resource is missing.
    pub fn restore_resource(
        &mut self,
        id: &ResourceId,
        amount: u32,
    ) -> CoreResult<ResourceRestoreOutcome> {
        self.state.resources_mut().restore(id, amount)
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
