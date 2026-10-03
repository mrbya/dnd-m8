use dnd_m8_ruleset::RulesetCharacterSummary;

use super::HitPointState;

/// Read-only character projection composed from ruleset data and runtime state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterSummary {
    /// Character name.
    pub name: String,

    /// Total character level.
    pub level: u8,

    /// User-facing class or multiclass description.
    pub class_name: String,

    /// Current ordinary hit points.
    pub current_hit_points: u32,

    /// Current maximum hit points.
    pub max_hit_points: u32,

    /// Current temporary hit points.
    pub temporary_hit_points: u32,

    /// Current armor class.
    pub armor_class: u16,

    /// Current proficiency bonus.
    pub proficiency_bonus: i16,

    /// Primary movement speed in feet.
    pub speed: u16,
}

impl CharacterSummary {
    /// Combines ruleset-derived information with common runtime state.
    #[must_use]
    pub(crate) fn from_ruleset(
        ruleset: RulesetCharacterSummary,
        hit_points: &HitPointState,
    ) -> Self {
        Self {
            name: ruleset.name,
            level: ruleset.level,
            class_name: ruleset.class_name,
            current_hit_points: hit_points.current(),
            max_hit_points: hit_points.maximum(),
            temporary_hit_points: hit_points.temporary(),
            armor_class: ruleset.armor_class,
            proficiency_bonus: ruleset.proficiency_bonus,
            speed: ruleset.speed,
        }
    }
}
