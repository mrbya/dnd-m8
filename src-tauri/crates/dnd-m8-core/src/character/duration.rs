use std::num::NonZeroU32;

use crate::{CoreError, CoreResult};

/// Remaning lifetime of an active effect instance.
///
/// Core tracks explicit countdouws without interpreting ruleset time units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectDuration {
    /// Remains active until explicitly removed.
    UntilRemoved,

    /// Expires after the remaining rounds have elapsed.
    Rounds {
        /// Positive number of remaining rounds.
        remaining: NonZeroU32,
    },
}

impl EffectDuration {
    /// Constructs a positive round countdown.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::InvalidEffectDuration`] when `remaining` is zero.
    pub fn rounds(remaining: u32) -> CoreResult<Self> {
        let remaining = NonZeroU32::new(remaining)
            .ok_or_else(|| Box::new(CoreError::InvalidEffectDuration { rounds: remaining }))?;

        Ok(Self::Rounds { remaining })
    }

    /// Returns the remaining rounds, or none for an indefinite duration.
    #[must_use]
    pub const fn remaining_rounds(&self) -> Option<u32> {
        match *self {
            Self::UntilRemoved => None,
            Self::Rounds { remaining } => Some(remaining.get()),
        }
    }

    /// Calculates the duration after an explicit round advancement.
    ///
    /// A missing remaining duration indicates expiration.
    pub(super) fn advance_rounds(self, rounds: u32) -> DurationAdvance {
        match self {
            Self::UntilRemoved => DurationAdvance {
                elapsed_rounds: 0,
                remaining_duration: Some(Self::UntilRemoved),
            },
            Self::Rounds { remaining } => {
                let previous = remaining.get();
                let elapsed_rounds = rounds.min(previous);
                let next = previous.saturating_sub(rounds);

                DurationAdvance {
                    elapsed_rounds,
                    remaining_duration: NonZeroU32::new(next).map(|new_remainder| Self::Rounds {
                        remaining: new_remainder,
                    }),
                }
            }
        }
    }
}

/// Result of advancing a duration without mutating stored runtime state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct DurationAdvance {
    /// Rounds consumed from the finite countdown.
    pub(super) elapsed_rounds: u32,
    /// Remaining duration, or none when the countdown expired.
    pub(super) remaining_duration: Option<EffectDuration>,
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::{CoreResult, EffectDuration};

    #[test]
    fn rounds_rejects_zero() {
        EffectDuration::rounds(0).expect_err("zero rounds should be rejected");
    }

    #[test]
    fn rounds_constructs_positive_duration() -> CoreResult<()> {
        let duration = EffectDuration::rounds(3)?;

        assert_eq!(duration.remaining_rounds(), Some(3));

        Ok(())
    }

    #[test]
    fn zero_advancement_preserves_finite_duration() -> CoreResult<()> {
        let duration = EffectDuration::rounds(3)?;
        let advance = duration.advance_rounds(0);

        assert_eq!(advance.elapsed_rounds, 0);
        assert_eq!(advance.remaining_duration, Some(duration));

        Ok(())
    }

    #[test]
    fn partial_advancement_preserves_remaining_duration() -> CoreResult<()> {
        let advance = EffectDuration::rounds(3)?.advance_rounds(2);

        assert_eq!(advance.elapsed_rounds, 2);
        assert_eq!(advance.remaining_duration, Some(EffectDuration::rounds(1)?));

        Ok(())
    }

    #[test]
    fn exact_advancement_expires_duration() -> CoreResult<()> {
        let advance = EffectDuration::rounds(3)?.advance_rounds(3);

        assert_eq!(advance.elapsed_rounds, 3);
        assert_eq!(advance.remaining_duration, None);

        Ok(())
    }

    #[test]
    fn overshoot_advancement_expires_duration_without_overflow() -> CoreResult<()> {
        let advance = EffectDuration::rounds(3)?.advance_rounds(u32::MAX);

        assert_eq!(advance.elapsed_rounds, 3);
        assert_eq!(advance.remaining_duration, None);

        Ok(())
    }

    #[test]
    fn advancing_indefinite_duration_preserves_it() {
        let advance = EffectDuration::UntilRemoved.advance_rounds(u32::MAX);

        assert_eq!(advance.elapsed_rounds, 0);
        assert_eq!(
            advance.remaining_duration,
            Some(EffectDuration::UntilRemoved)
        );
    }

    proptest! {
        #[test]
        fn finite_duration_accounting_is_conserved(
            original in 1_u32..=u32::MAX,
            requested in any::<u32>(),
        ) {
            let duration = EffectDuration::rounds(original)
                .expect("generated positive duration should be valid");
            let advance = duration.advance_rounds(requested);
            let expected_elapsed = original.min(requested);
            let expected_remaining = original.saturating_sub(expected_elapsed);

            prop_assert_eq!(advance.elapsed_rounds, expected_elapsed);
            prop_assert_eq!(
                advance.remaining_duration.map(|remaining| remaining.remaining_rounds()),
                (expected_remaining > 0).then_some(Some(expected_remaining)),
            );
            prop_assert_eq!(
                u64::from(advance.elapsed_rounds).saturating_add(u64::from(expected_remaining)),
                u64::from(original),
            );
        }
    }
}
