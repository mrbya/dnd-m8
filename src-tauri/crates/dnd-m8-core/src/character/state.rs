use super::hitpoints::HitPointState;

/// Ruleset-independent state that changes while a character is being played.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterState {
    /// Current hit-point state.
    hit_points: HitPointState,
}

impl CharacterState {
    /// Constructs character runtime state.
    #[must_use]
    pub const fn new(hit_points: HitPointState) -> Self {
        Self { hit_points }
    }

    /// Returns current hit-point state.
    #[must_use]
    pub const fn hit_points(&self) -> &HitPointState {
        &self.hit_points
    }

    /// Returns mutable hit-point state for aggregate operations.
    #[allow(dead_code)] // TODO: remove once operations are itegrated.
    pub(crate) const fn hit_points_mut(&mut self) -> &mut HitPointState {
        &mut self.hit_points
    }
}
