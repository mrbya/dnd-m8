use dnd_m8_ruleset::{
    CharacterSummary, Ruleset, RulesetCharacterData, RulesetError, RulesetId, RulesetMetadata,
    RulesetResult,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{Character, CharacterId};

/// Schema understood by [`TestRuleset`].
pub const TEST_SCHEMA: &str = "org.dnd-m8.test.character@1";

/// Error returned for an unsupported test schema.
pub const INVALID_SCHEMA_MESSAGE: &str = "unsupported test character schema";

/// Character name contained in the test fixture.
const TEST_CHARACTER_NAME: &str = "Bruce Lee";

/// Minimal ruleset implementation used by core unit tests.
#[derive(Debug)]
pub struct TestRuleset {
    /// Ruleset metadata.
    metadata: RulesetMetadata,
}

impl TestRuleset {
    /// Constructs the test ruleset.
    pub fn new() -> RulesetResult<Self> {
        Ok(Self {
            metadata: RulesetMetadata {
                id: RulesetId::new("org.dnd-m8.test", "1.0.0")?,
                display_name: String::from("Test ruleset"),
                description: String::from("Core test fixture"),
            },
        })
    }

    /// Returns the test ruleset id.
    #[must_use]
    pub const fn id(&self) -> &RulesetId {
        &self.metadata.id
    }
}

impl Ruleset for TestRuleset {
    fn metadata(&self) -> &RulesetMetadata {
        &self.metadata
    }

    fn summarize_character(
        &self,
        character: &dnd_m8_ruleset::RulesetCharacterData,
    ) -> RulesetResult<dnd_m8_ruleset::CharacterSummary> {
        if character.schema() != TEST_SCHEMA {
            return Err(RulesetError::Generic {
                msg: String::from(INVALID_SCHEMA_MESSAGE),
            });
        }

        let name = character
            .data()
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| RulesetError::Generic {
                msg: String::from("test character has no valid name"),
            })?;

        let mut summary = expected_summary();
        name.clone_into(&mut summary.name);

        Ok(summary)
    }
}

/// Constructs a test character with the requested schema.
#[must_use]
pub fn test_character(id: CharacterId, ruleset_id: RulesetId, schema: &str) -> Character {
    Character::new(
        id,
        ruleset_id,
        RulesetCharacterData::new(
            schema,
            json!({
                "name": TEST_CHARACTER_NAME,
            }),
        ),
    )
}

/// Returns the deterministic character identifier used by tests.
#[must_use]
pub fn test_character_id() -> CharacterId {
    CharacterId::from_uuid(Uuid::from_u128(1))
}

/// Returns the summary expected from a valid test character.
pub fn expected_summary() -> CharacterSummary {
    CharacterSummary {
        name: String::from(TEST_CHARACTER_NAME),
        level: 19,
        class_name: String::from("Monk"),
        current_hit_points: 1,
        max_hit_points: 99,
        armor_class: 18,
        proficiency_bonus: 6,
        speed: 30,
    }
}
