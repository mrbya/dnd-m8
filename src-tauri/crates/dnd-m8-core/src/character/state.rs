use crate::ResourcePools;

use super::hitpoints::HitPointState;

/// Ruleset-independent state that changes while a character is being played.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterState {
    /// Current hit-point state.
    hit_points: HitPointState,
    /// Current generic resource pools.
    resources: ResourcePools,
}

impl CharacterState {
    /// Constructs character runtime state.
    #[must_use]
    pub fn new(hit_points: HitPointState) -> Self {
        Self {
            hit_points,
            resources: ResourcePools::new(),
        }
    }

    /// Constructs character runtime state with preconfigured resource pools.
    #[must_use]
    pub const fn with_resources(hit_points: HitPointState, resources: ResourcePools) -> Self {
        Self {
            hit_points,
            resources,
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
}
