use dnd_m8_ruleset::{RulesetCharacterData, RulesetId};

/// Active condition runtime state.
mod conditions;
/// Hit-point runtime state and transitions.
mod hitpoints;
/// Shared type identifiers for runtime state.
mod ids;
/// Generic limited-resource runtime state.
mod resources;
/// Shared storage for active condition and effect instances.
mod runtime_instance;
/// Ruleset-independent character runtime state.
mod state;
/// Combined character read model.
mod summary;

// Re-exports.
pub use conditions::{
    ActiveCondition, ActiveConditions, ConditionApplyOutcome, ConditionId, ConditionInstanceId,
    ConditionKind, ConditionRemoveOutcome,
};
pub use hitpoints::{DamageOutcome, HealingOutcome, HitPointState};
pub use ids::{DefinitionId, InstanceId};
pub use resources::{
    ResourceId, ResourcePool, ResourcePools, ResourceRestoreOutcome, ResourceSpendOutcome,
};
pub use runtime_instance::{Instance, Instances};
pub use state::CharacterState;
pub use summary::CharacterSummary;

use crate::CoreResult;

/// Compile-time domain marker for character identifiers.
///
/// This type has no runtime value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CharacterKind {}

/// Unique identifier for characters managed by the app.
pub type CharacterId = InstanceId<CharacterKind>;

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

    /// Applies a condition to this character.
    ///
    /// Repeated instances of the same condition are tracked independently.
    ///
    /// # Errors
    ///
    /// Returns an error if the generated runtime application identifier collides
    /// with an existing application.
    pub fn apply_condition(
        &mut self,
        condition_id: ConditionId,
    ) -> CoreResult<ConditionApplyOutcome> {
        self.state.conditions_mut().apply(condition_id)
    }

    /// Removes one active condition application.
    ///
    /// # Errors
    ///
    /// Returns an error when the application does not exist.
    pub fn remove_condition(
        &mut self,
        instance_id: ConditionInstanceId,
    ) -> CoreResult<ConditionRemoveOutcome> {
        self.state.conditions_mut().remove(instance_id)
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
