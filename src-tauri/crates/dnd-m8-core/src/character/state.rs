use crate::{character::ActiveEffects, ActiveConditions, EffectInstanceId, ResourcePools};

use super::hitpoints::HitPointState;

/// Ruleset-independent state that changes while a character is being played.
///
/// Ordinary hit points and resource amounts do not exceed their maxima.
/// Finite active effects always have a positive remaining duration.
/// Concentration references an active effect on this character.
/// Character operations clear concentration when its effect is removed
/// or expires.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterState {
    /// Current hit-point state.
    hit_points: HitPointState,
    /// Current generic resource pools.
    resources: ResourcePools,
    /// Currently active condition instances.
    conditions: ActiveConditions,
    /// Currently active effects.
    effects: ActiveEffects,
    /// Effect instance currently sustained through concentration.
    concentration: Option<EffectInstanceId>,
}

impl CharacterState {
    /// Constructs character runtime state.
    #[must_use]
    pub fn new(hit_points: HitPointState) -> Self {
        Self {
            hit_points,
            resources: ResourcePools::new(),
            conditions: ActiveConditions::new(),
            effects: ActiveEffects::new(),
            concentration: None,
        }
    }

    /// Constructs character runtime state with preconfigured resource pools.
    #[must_use]
    pub fn with_resources(hit_points: HitPointState, resources: ResourcePools) -> Self {
        Self {
            hit_points,
            resources,
            conditions: ActiveConditions::new(),
            effects: ActiveEffects::new(),
            concentration: None,
        }
    }

    /// Returns current hit-point state.
    #[must_use]
    pub const fn hit_points(&self) -> &HitPointState {
        &self.hit_points
    }

    /// Returns mutable hit-point state for aggregate operations.
    pub(crate) const fn hit_points_mut(&mut self) -> &mut HitPointState {
        &mut self.hit_points
    }

    /// Returns the character's generic resources.
    #[must_use]
    pub const fn resources(&self) -> &ResourcePools {
        &self.resources
    }

    /// Returns mutable resources for aggregate operations.
    pub(crate) const fn resources_mut(&mut self) -> &mut ResourcePools {
        &mut self.resources
    }

    /// Returns active condition instances.
    #[must_use]
    pub const fn conditions(&self) -> &ActiveConditions {
        &self.conditions
    }

    /// Returns active effect instances.
    #[must_use]
    pub const fn effects(&self) -> &ActiveEffects {
        &self.effects
    }

    /// Returns the effect instance currently sustained through concentration.
    #[must_use]
    pub const fn concentration(&self) -> Option<EffectInstanceId> {
        self.concentration
    }

    /// Updates concentration after an aggregate transition validates its target.
    pub(crate) const fn set_concentration(&mut self, concentration: Option<EffectInstanceId>) {
        self.concentration = concentration;
    }

    /// Returns mutable conditions for aggregate operations.
    pub(crate) const fn conditions_mut(&mut self) -> &mut ActiveConditions {
        &mut self.conditions
    }

    /// Returns effects for aggregate transitions.
    pub(crate) const fn effects_mut(&mut self) -> &mut ActiveEffects {
        &mut self.effects
    }
}
