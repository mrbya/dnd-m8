use dnd_m8_ruleset::{RulesetCharacterData, RulesetId};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::{
    ActiveCondition, ActiveConditions, ActiveEffect, ActiveEffects, Character, CharacterId,
    CharacterState, ConditionId, ConditionInstanceId, CoreError, EffectDuration, EffectId,
    EffectInstanceId, HitPointState, Instance, PersistenceError, PersistenceResult, ResourceId,
    ResourcePool, ResourcePools,
};

/// Current saved-character format version.
const FORMAT_VERSION: u32 = 1;

/// Versioned envelope surrounding a character payload.
#[derive(Debug, Serialize, Deserialize)]
struct SavedCharacter<T> {
    /// Application-owned persistence format version.
    format_version: u32,
    /// Character payload intercepted according to the format version.
    character: T,
}

/// Complete character record for persistence format version one.
#[derive(Debug, Serialize, Deserialize)]
struct CharacterRecordV1 {
    /// Persistent character identity.
    id: Uuid,
    /// Exact identity of the required ruleset.
    ruleset: RulesetRecord,
    /// Opaque ruleset-owned character definition.
    ruleset_data: RulesetDataRecord,
    /// Complete common runtime state.
    state: StateRecord,
}

/// Exact versioned ruleset identity.
#[derive(Debug, Serialize, Deserialize)]
struct RulesetRecord {
    /// Globally namespaced ruleset family.
    family: String,
    /// Exact semantic version.
    version: String,
}

/// Ruleset-owned character definition.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RulesetDataRecord {
    /// Ruleset-provided schema identifier.
    schema: String,
    /// Opaque JSON interpreted by the owning ruleset.
    data: Value,
}

/// Saved common runtime state.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StateRecord {
    /// Ordinary and temporary hit points.
    hit_points: HitPointRecord,
    /// Independently identified resource pools.
    resources: Vec<ResourceRecord>,
    /// Independently identified condition instances.
    conditions: Vec<ConditionRecord>,
    /// Independently identified effect instances.
    effects: Vec<EffectRecord>,
    /// Specific effect instance sustained by concentration.
    concentration: Option<Uuid>,
}

/// Saved hit-point values.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct HitPointRecord {
    /// Current ordinary hit points.
    current: u32,
    /// Maximum ordinary hit points.
    maximum: u32,
    /// Current temporary hit points.
    temporary: u32,
}

/// Saved resource pool.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ResourceRecord {
    /// Ruleset-defined resource identity.
    id: String,
    /// Currently available amount.
    current: u32,
    /// Maximum available amount.
    maximum: u32,
}

/// Saved condition instance.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ConditionRecord {
    /// Persistent runtime instance identity.
    instance_id: Uuid,
    /// Ruleset-defined condition identity.
    condition_id: String,
}

/// Saved effect instance.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct EffectRecord {
    /// Persistent runtime instance identity.
    instance_id: Uuid,
    /// Ruleset-defined effect identity.
    effect_id: String,
    /// Remaining explicit duration.
    duration: DurationRecord,
}

/// Saved duration representation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum DurationRecord {
    /// Remains active until explicitly removed.
    UntilRemoved,
    /// Positive remaining round countdown.
    Rounds {
        /// Remaining rounds, validated during reconstruction.
        remaining: u32,
    },
}

/// Encodes a complete character using the current persistence format.
///
/// # Errors
///
/// Returns [`PersistenceError::Json`] when serialization fails.
pub fn encode_character(character: &Character) -> PersistenceResult<String> {
    let record = CharacterRecordV1::from(character);
    let saved = SavedCharacter {
        format_version: FORMAT_VERSION,
        character: record,
    };

    Ok(serde_json::to_string_pretty(&saved)?)
}

/// Decodes and validates a complete saved character.
///
/// Ruleset availability is not required to reconstruct the record.
/// Interpreting its opaque definition remains a separate ruleset operation.
///
/// # Errors
///
/// Returns an error for malformed JSON, unsupported format versions,
/// invalid ruleset identities, or invalid runtime state.
pub fn decode_character(json: &str) -> PersistenceResult<Character> {
    let saved: SavedCharacter<Value> = serde_json::from_str(json)?;

    if saved.format_version != FORMAT_VERSION {
        return Err(PersistenceError::UnsupportedVersion {
            version: saved.format_version,
        });
    }

    let record: CharacterRecordV1 = serde_json::from_value(saved.character)?;

    record.into_character()
}

impl From<&Character> for CharacterRecordV1 {
    fn from(character: &Character) -> Self {
        Self {
            id: *character.id().as_uuid(),
            ruleset: RulesetRecord {
                family: character.ruleset_id().family().to_owned(),
                version: character.ruleset_id().version().to_string(),
            },
            ruleset_data: RulesetDataRecord {
                schema: character.ruleset_data().schema().to_owned(),
                data: character.ruleset_data().data().clone(),
            },
            state: StateRecord::from(character.state()),
        }
    }
}

impl CharacterRecordV1 {
    /// Reconstructs a character through validated domain constructors.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid ruleset identity or runtime state.
    fn into_character(self) -> PersistenceResult<Character> {
        let ruleset_id = RulesetId::new(self.ruleset.family, self.ruleset.version)?;
        let ruleset_data =
            RulesetCharacterData::new(self.ruleset_data.schema, self.ruleset_data.data);
        let state = self.state.into_state()?;

        Ok(Character::new(
            CharacterId::from_uuid(self.id),
            ruleset_id,
            ruleset_data,
            state,
        ))
    }
}

impl From<&CharacterState> for StateRecord {
    fn from(state: &CharacterState) -> Self {
        let hit_points = state.hit_points();

        Self {
            hit_points: HitPointRecord {
                current: hit_points.current(),
                maximum: hit_points.maximum(),
                temporary: hit_points.temporary(),
            },
            resources: state
                .resources()
                .iter()
                .map(|pool| ResourceRecord {
                    id: pool.id().as_str().to_owned(),
                    current: pool.current(),
                    maximum: pool.maximum(),
                })
                .collect(),
            conditions: state
                .conditions()
                .iter()
                .map(|condition| ConditionRecord {
                    instance_id: *condition.instance_id().as_uuid(),
                    condition_id: condition.definition_id().as_str().to_owned(),
                })
                .collect(),
            effects: state
                .effects()
                .iter()
                .map(|effect| EffectRecord {
                    instance_id: *effect.instance_id().as_uuid(),
                    effect_id: effect.effect_id().as_str().to_owned(),
                    duration: DurationRecord::from(effect.duration()),
                })
                .collect(),
            concentration: state
                .concentration()
                .map(|instance_id| *instance_id.as_uuid()),
        }
    }
}

impl StateRecord {
    /// Reconstructs all runtime collections before validating concentration.
    ///
    /// # Errors
    ///
    /// Returns an error when any saved component violates domain invariants.
    fn into_state(self) -> PersistenceResult<CharacterState> {
        let hit_points = HitPointState::new(
            self.hit_points.current,
            self.hit_points.maximum,
            self.hit_points.temporary,
        )?;

        let mut resources = ResourcePools::new();

        for resource in self.resources {
            let id = ResourceId::new(resource.id)?;
            let pool = ResourcePool::new(id, resource.current, resource.maximum)?;

            resources.insert(pool)?;
        }

        let mut conditions = ActiveConditions::new();

        for condition in self.conditions {
            let instance_id = ConditionInstanceId::from_uuid(condition.instance_id);
            let condition_id = ConditionId::new(condition.condition_id)?;

            conditions.insert(ActiveCondition::from_parts(instance_id, condition_id))?;
        }

        let mut effects = ActiveEffects::new();

        for effect in self.effects {
            let instance_id = EffectInstanceId::from_uuid(effect.instance_id);
            let effect_id = EffectId::new(effect.effect_id)?;
            let duration = effect.duration.into_duration()?;

            effects.insert(ActiveEffect::from_parts(instance_id, effect_id, duration))?;
        }

        let concentration = self.concentration.map(EffectInstanceId::from_uuid);

        Ok(CharacterState::from_parts(
            hit_points,
            resources,
            conditions,
            effects,
            concentration,
        )?)
    }
}

impl From<EffectDuration> for DurationRecord {
    fn from(duration: EffectDuration) -> Self {
        match duration {
            EffectDuration::UntilRemoved => Self::UntilRemoved,
            EffectDuration::Rounds { remaining } => Self::Rounds {
                remaining: remaining.get(),
            },
        }
    }
}

impl DurationRecord {
    /// Reconstructs a validated effect duration.
    ///
    /// # Errors
    ///
    /// Returns an error when a finite duration contains zero rounds.
    fn into_duration(self) -> Result<EffectDuration, Box<CoreError>> {
        match self {
            Self::UntilRemoved => Ok(EffectDuration::UntilRemoved),
            Self::Rounds { remaining } => EffectDuration::rounds(remaining),
        }
    }
}
