use crate::{CoreError, CoreResult};

/// Current ruleset-independent hit-point state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HitPointState {
    /// Current ordinary hit points.
    current: u32,
    /// Current maximum hit points.
    maximum: u32,
    /// Current temporary hit points.
    temporary: u32,
}

impl HitPointState {
    /// Constructs validated hit-point state.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::InvalidHitPointState`] when `current` exceeds
    /// `maximum`.
    pub fn new(current: u32, maximum: u32, temporary: u32) -> CoreResult<Self> {
        if current > maximum {
            return Err(Box::new(CoreError::InvalidHitPointState {
                current,
                maximum,
            }));
        }

        Ok(Self {
            current,
            maximum,
            temporary,
        })
    }

    /// Returns current ordinary hit points.
    #[must_use]
    pub const fn current(&self) -> u32 {
        self.current
    }

    /// Returns current maximum hit points.
    #[must_use]
    pub const fn maximum(&self) -> u32 {
        self.maximum
    }

    /// Returns current temporary hit points.
    #[must_use]
    pub const fn temporary(&self) -> u32 {
        self.temporary
    }

    /// Applies damage, consuming temporary hit points before ordinary hit
    /// points.
    #[must_use]
    pub fn take_damage(&mut self, amount: u32) -> DamageOutcome {
        let temporary_lost = amount.min(self.temporary);
        self.temporary = self.temporary.saturating_sub(temporary_lost);

        let remaining = amount.saturating_sub(temporary_lost);
        let hit_points_lost = remaining.min(self.current);
        self.current = self.current.saturating_sub(hit_points_lost);

        DamageOutcome {
            requested: amount,
            temporary_lost,
            hit_points_lost,
            excess: remaining.saturating_sub(hit_points_lost),
        }
    }

    /// Restores ordinary hit points without exceeding the current maximum.
    #[must_use]
    pub fn heal(&mut self, amount: u32) -> HealingOutcome {
        let available = self.maximum.saturating_sub(self.current);
        let restored = amount.min(available);

        self.current = self.current.saturating_add(restored);

        HealingOutcome {
            requested: amount,
            restored,
            excess: amount.saturating_sub(restored),
        }
    }

    /// Replaces the current temporary hit points.
    ///
    /// Returns the previously held amount.
    #[must_use]
    pub const fn replace_temporary(&mut self, amount: u32) -> u32 {
        std::mem::replace(&mut self.temporary, amount)
    }
}

/// Result of applying damage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DamageOutcome {
    /// Damage requested by the caller.
    pub requested: u32,
    /// Damage absorbed by temporary hit points.
    pub temporary_lost: u32,
    /// Damage removed from ordinary hit points.
    pub hit_points_lost: u32,
    /// Damage remaining after the character reached zero hit points.
    pub excess: u32,
}

/// Result of applying healing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HealingOutcome {
    /// Healing requested by the caller.
    pub requested: u32,
    /// Ordinary hit points actually restored.
    pub restored: u32,
    /// Healing unused because the character reached maximum hit points.
    pub excess: u32,
}

#[cfg(test)]
mod tests {
    use crate::{CoreError, CoreResult, DamageOutcome, HealingOutcome, HitPointState};
    use pretty_assertions::assert_eq;

    #[test]
    fn state_accepts_current_equal_max() -> CoreResult<()> {
        let hit_points = HitPointState::new(10, 10, 0)?;

        assert_eq!(hit_points.current(), 10);
        assert_eq!(hit_points.maximum(), 10);

        Ok(())
    }

    #[test]
    fn state_accepts_current_max_zero() -> CoreResult<()> {
        let hit_points = HitPointState::new(0, 0, 0)?;

        assert_eq!(hit_points.current(), 0);
        assert_eq!(hit_points.maximum(), 0);

        Ok(())
    }

    #[test]
    fn state_rejects_current_hit_points_above_maximum() {
        let result = HitPointState::new(11, 10, 0);

        assert!(matches!(
            result,
            Err(error)
                if matches!(
                    error.as_ref(),
                    CoreError::InvalidHitPointState {
                        current: 11,
                        maximum: 10,
                    }
                )
        ));
    }

    #[test]
    fn damage_consumes_temporary_hit_points_first() -> CoreResult<()> {
        let mut hit_points = HitPointState::new(20, 20, 5)?;

        let outcome = hit_points.take_damage(8);

        assert_eq!(hit_points.current(), 17);
        assert_eq!(hit_points.temporary(), 0);
        assert_eq!(
            outcome,
            DamageOutcome {
                requested: 8,
                temporary_lost: 5,
                hit_points_lost: 3,
                excess: 0,
            }
        );

        Ok(())
    }

    #[test]
    fn damage_clamps_hp_at_zero() -> CoreResult<()> {
        let mut hit_points = HitPointState::new(10, 15, 0)?;

        let outcome = hit_points.take_damage(17);

        assert_eq!(hit_points.current(), 0);
        assert_eq!(
            outcome,
            DamageOutcome {
                requested: 17,
                temporary_lost: 0,
                hit_points_lost: 10,
                excess: 7,
            }
        );

        Ok(())
    }

    #[test]
    fn healing_clamps_at_max_and_doesnt_alter_temporary() -> CoreResult<()> {
        let mut hit_points = HitPointState::new(10, 15, 0)?;

        let outcome = hit_points.heal(7);

        assert_eq!(hit_points.current(), 15);
        assert_eq!(hit_points.temporary(), 0);
        assert_eq!(
            outcome,
            HealingOutcome {
                requested: 7,
                restored: 5,
                excess: 2,
            }
        );

        Ok(())
    }

    #[test]
    fn replacing_temporary_returns_previous_value() -> CoreResult<()> {
        let mut hit_points = HitPointState::new(10, 15, 8)?;

        assert_eq!(hit_points.temporary(), 8);

        let outcome = hit_points.replace_temporary(4);

        assert_eq!(hit_points.temporary(), 4);
        assert_eq!(outcome, 8);

        Ok(())
    }
}
