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
