//! Character data and state API bound to one ruleset.

use serde_json::Value;

/// Opaque character definition owned by a concrete ruleset.
///
/// The application may store and forward this value, but only
/// the matching ruleset implementation may interpret it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RulesetCharacterData {
    /// Character schema.
    schema: String,
    /// Character json data.
    data: Value,
}

impl RulesetCharacterData {
    /// Wraps a ruleset-specific character definition.
    pub fn new(schema: impl Into<String>, data: Value) -> Self {
        Self {
            schema: schema.into(),
            data,
        }
    }

    /// Returns ruleset-specific payload schema.
    #[must_use]
    pub fn schema(&self) -> &str {
        &self.schema
    }

    /// Returns the opaque JSON payload.
    #[must_use]
    pub const fn data(&self) -> &Value {
        &self.data
    }
}

/// Read-only character data required by the walking skeleton.
///
/// This is a projection for application consumption, not the persistent
/// character model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterSummary {
    /// Character name.
    pub name: String,

    /// Total character level.
    pub level: u8,

    /// User-facing class or multiclass description.
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
