use crate::{EffectInstanceId, EffectRemoveOutcome};

/// Result of an explicit concentration transition.
///
/// Ending or replacing concentration removes the previously concentrated
/// effect instance. Repeating the current state is a successful no-op.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcentrationChangeOutcome {
    /// Concentrated effect instance before the operation.
    pub previous: Option<EffectInstanceId>,
    /// Concentrated effect instance after the operation.
    pub current: Option<EffectInstanceId>,
    /// Effect removed because its concentration ended.
    pub removed_effect: Option<EffectRemoveOutcome>,
}
