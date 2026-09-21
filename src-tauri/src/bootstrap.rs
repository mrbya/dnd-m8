use dnd_m8_core::{AppState as CoreAppState, Character, CharacterId, CharacterService, CoreError};
use dnd_m8_ruleset::{
    CharacterSummary, Ruleset, RulesetCharacterData, RulesetError, RulesetId, RulesetMetadata,
    RulesetRegistry, RulesetResult,
};
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;

use crate::state::ShellState;

/// Schema understood by the private demo ruleset.
const DEMO_CHARACTER_SCHEMA: &str = "org.dnd-m8.demo.character@1";

/// Failure encountered while constructing initial application state.
#[derive(Debug, Error)]
pub enum BootstrapError {
    /// Failed to configure a ruleset.
    #[error(transparent)]
    Ruleset(#[from] RulesetError),

    /// Failed to configure the application core.
    #[error(transparent)]
    Core(#[from] Box<CoreError>),
}

/// Private walking-skeleton ruleset.
///
/// This is deliberately not part of the public ruleset ecosystem.
struct DemoRuleset {
    /// Ruleset metadata.
    metadata: RulesetMetadata,
}

impl DemoRuleset {
    /// Constructs the demo ruleset.
    fn new() -> RulesetResult<Self> {
        Ok(Self {
            metadata: RulesetMetadata {
                id: RulesetId::new("org.dnd-m8.demo", "1.0.0")?,
                display_name: String::from("D&D Mate Demo"),
                description: String::from("Temporary ruleset used by the walking skeleton"),
            },
        })
    }
}

/// Payload understood only by [`DemoRuleset`].
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DemoCharacter {
    /// Character name.
    name: String,

    /// Total level.
    level: u8,

    /// Class description.
    class_name: String,

    /// Current hit points.
    current_hit_points: i32,

    /// Maximum hit points.
    max_hit_points: u32,

    /// Armor class.
    armor_class: u16,

    /// Proficiency bonus.
    proficiency_bonus: i16,

    /// Movement speed in feet.
    speed: u16,
}

impl Ruleset for DemoRuleset {
    fn metadata(&self) -> &RulesetMetadata {
        &self.metadata
    }

    fn summarize_character(
        &self,
        character: &RulesetCharacterData,
    ) -> RulesetResult<CharacterSummary> {
        if character.schema() != DEMO_CHARACTER_SCHEMA {
            return Err(RulesetError::Generic {
                msg: format!("unsupported demo character schema `{}`", character.schema()),
            });
        }

        let character: DemoCharacter =
            serde_json::from_value(character.data().clone()).map_err(|error| {
                RulesetError::Generic {
                    msg: format!("invalid demo character payload: {error}"),
                }
            })?;

        Ok(CharacterSummary {
            name: character.name,
            level: character.level,
            class_name: character.class_name,
            current_hit_points: character.current_hit_points,
            max_hit_points: character.max_hit_points,
            armor_class: character.armor_class,
            proficiency_bonus: character.proficiency_bonus,
            speed: character.speed,
        })
    }
}

/// Constructs the initial state managed by Tauri.
///
/// # Errors
///
/// Returns an error if the embedded demo ruleset or character cannot be
/// registered.
pub fn build_state() -> Result<ShellState, BootstrapError> {
    let ruleset = DemoRuleset::new()?;
    let ruleset_id = ruleset.metadata().id.clone();

    let mut registry = RulesetRegistry::new();
    registry.register(ruleset)?;

    let character_id = CharacterId::new();

    let character = Character::new(
        character_id,
        ruleset_id,
        RulesetCharacterData::new(
            DEMO_CHARACTER_SCHEMA,
            json!({
                "name": "Mira Ashfall",
                "level": 5,
                "className": "Life Domain Cleric",
                "currentHitPoints": 31,
                "maxHitPoints": 38,
                "armorClass": 18,
                "proficiencyBonus": 3,
                "speed": 30
            }),
        ),
    );

    let mut characters = CharacterService::new(registry);
    characters.add_character(character)?;

    let core = CoreAppState::new(characters);

    Ok(ShellState::new(core, character_id))
}
