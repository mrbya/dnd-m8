use dnd_m8_core::CharacterId;
use dnd_m8_ruleset::CharacterSummary;
use serde::Serialize;

/// Serializable character summary returned to the frontend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterSummaryDto {
    /// Stable character identifier.
    pub id: String,

    /// Character name.
    pub name: String,

    /// Total character level.
    pub level: u8,

    /// User-facing class description.
    pub class_name: String,

    /// Current hit points.
    pub current_hit_points: i32,

    /// Maximum hit points.
    pub max_hit_points: u32,

    /// Current armor class.
    pub armor_class: u16,

    /// Current proficiency bonus.
    pub proficiency_bonus: i16,

    /// Primary movement speed in feet.
    pub speed: u16,
}

impl CharacterSummaryDto {
    /// Translates a domain summary into its frontend representation.
    #[must_use]
    pub fn from_domain(id: CharacterId, summary: CharacterSummary) -> Self {
        Self {
            id: id.to_string(),
            name: summary.name,
            level: summary.level,
            class_name: summary.class_name,
            current_hit_points: summary.current_hit_points,
            max_hit_points: summary.max_hit_points,
            armor_class: summary.armor_class,
            proficiency_bonus: summary.proficiency_bonus,
            speed: summary.speed,
        }
    }
}
