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

#[cfg(test)]
mod tests {
    use std::error::Error;

    use dnd_m8_ruleset::{RulesetCharacterData, RulesetId};
    use serde_json::{json, Value};
    use uuid::Uuid;

    use crate::{
        decode_character, encode_character, ActiveCondition, ActiveConditions, ActiveEffect,
        ActiveEffects, Character, CharacterId, CharacterState, ConditionId, ConditionInstanceId,
        EffectDuration, EffectId, EffectInstanceId, HitPointState, PersistenceError, ResourceId,
        ResourcePool, ResourcePools,
    };

    type TestResult<T = ()> = Result<T, Box<dyn Error>>;

    fn character_with_complete_runtime_state() -> TestResult<Character> {
        let character_id = CharacterId::from_uuid(Uuid::from_u128(1));
        let ruleset_id = RulesetId::new("org.dnd-m8.persistence-test", "1.2.3")?;
        let ruleset_data = RulesetCharacterData::new(
            "org.dnd-m8.persistence-test.character@1",
            json!({
                "name": "Persistence Test",
                "spellbook": {
                    "prepared": ["spell.example"],
                    "metadata": { "source": "fixture" },
                },
            }),
        );
        let hit_points = HitPointState::new(17, 23, 5)?;

        let mut resources = ResourcePools::new();
        resources.insert(ResourcePool::new(ResourceId::new("resource.first")?, 2, 3)?)?;
        resources.insert(ResourcePool::new(
            ResourceId::new("resource.second")?,
            4,
            7,
        )?)?;

        let condition_id = ConditionId::new("condition.repeated")?;
        let mut conditions = ActiveConditions::new();
        conditions.insert(ActiveCondition::from_parts(
            ConditionInstanceId::from_uuid(Uuid::from_u128(10)),
            condition_id.clone(),
        ))?;
        conditions.insert(ActiveCondition::from_parts(
            ConditionInstanceId::from_uuid(Uuid::from_u128(11)),
            condition_id,
        ))?;

        let concentrated_effect_id = EffectInstanceId::from_uuid(Uuid::from_u128(20));
        let mut effects = ActiveEffects::new();
        effects.insert(ActiveEffect::from_parts(
            concentrated_effect_id,
            EffectId::new("effect.finite")?,
            EffectDuration::rounds(3)?,
        ))?;
        effects.insert(ActiveEffect::from_parts(
            EffectInstanceId::from_uuid(Uuid::from_u128(21)),
            EffectId::new("effect.indefinite")?,
            EffectDuration::UntilRemoved,
        ))?;

        let state = CharacterState::from_parts(
            hit_points,
            resources,
            conditions,
            effects,
            Some(concentrated_effect_id),
        )?;

        Ok(Character::new(
            character_id,
            ruleset_id,
            ruleset_data,
            state,
        ))
    }

    fn encoded_value(character: &Character) -> TestResult<Value> {
        Ok(serde_json::from_str(&encode_character(character)?)?)
    }

    fn decode_value(value: &Value) -> Result<Character, PersistenceError> {
        decode_character(&serde_json::to_string(value).expect("fixture JSON should serialize"))
    }

    fn field_mut<'a>(value: &'a mut Value, pointer: &str) -> &'a mut Value {
        value
            .pointer_mut(pointer)
            .expect("encoded fixture should contain the requested field")
    }

    #[test]
    fn round_trip_preserves_complete_runtime_state() -> TestResult {
        let original = character_with_complete_runtime_state()?;
        let restored = decode_character(&encode_character(&original)?)?;

        assert_eq!(restored.id(), original.id());
        assert_eq!(restored.ruleset_id(), original.ruleset_id());
        assert_eq!(restored.ruleset_data(), original.ruleset_data());
        assert_eq!(restored.state(), original.state());

        Ok(())
    }

    #[test]
    fn rejects_hit_points_above_maximum() -> TestResult {
        let mut saved = encoded_value(&character_with_complete_runtime_state()?)?;
        *field_mut(&mut saved, "/character/state/hit_points/current") = json!(24);

        decode_value(&saved).expect_err("hit points above their maximum should be rejected");

        Ok(())
    }

    #[test]
    fn rejects_resource_amount_above_maximum() -> TestResult {
        let mut saved = encoded_value(&character_with_complete_runtime_state()?)?;
        *field_mut(&mut saved, "/character/state/resources/0/current") = json!(4);

        decode_value(&saved).expect_err("resource amount above its maximum should be rejected");

        Ok(())
    }

    #[test]
    fn rejects_duplicate_resource_identifier() -> TestResult {
        let mut saved = encoded_value(&character_with_complete_runtime_state()?)?;
        let duplicate = saved
            .pointer("/character/state/resources/0")
            .expect("encoded fixture should contain a resource")
            .clone();
        field_mut(&mut saved, "/character/state/resources")
            .as_array_mut()
            .expect("resources should be an array")
            .push(duplicate);

        decode_value(&saved).expect_err("duplicate resource should be rejected");

        Ok(())
    }

    #[test]
    fn rejects_duplicate_condition_instance_identifier() -> TestResult {
        let mut saved = encoded_value(&character_with_complete_runtime_state()?)?;
        let duplicate = saved
            .pointer("/character/state/conditions/0")
            .expect("encoded fixture should contain a condition")
            .clone();
        field_mut(&mut saved, "/character/state/conditions")
            .as_array_mut()
            .expect("conditions should be an array")
            .push(duplicate);

        decode_value(&saved).expect_err("duplicate condition instance should be rejected");

        Ok(())
    }

    #[test]
    fn rejects_duplicate_effect_instance_identifier() -> TestResult {
        let mut saved = encoded_value(&character_with_complete_runtime_state()?)?;
        let duplicate = saved
            .pointer("/character/state/effects/0")
            .expect("encoded fixture should contain an effect")
            .clone();
        field_mut(&mut saved, "/character/state/effects")
            .as_array_mut()
            .expect("effects should be an array")
            .push(duplicate);

        decode_value(&saved).expect_err("duplicate effect instance should be rejected");

        Ok(())
    }

    #[test]
    fn rejects_zero_finite_duration() -> TestResult {
        let mut saved = encoded_value(&character_with_complete_runtime_state()?)?;
        *field_mut(&mut saved, "/character/state/effects/0/duration/remaining") = json!(0);

        decode_value(&saved).expect_err("zero finite duration should be rejected");

        Ok(())
    }

    #[test]
    fn rejects_concentration_on_missing_effect() -> TestResult {
        let mut saved = encoded_value(&character_with_complete_runtime_state()?)?;
        *field_mut(&mut saved, "/character/state/concentration") = json!(Uuid::from_u128(99));

        decode_value(&saved).expect_err("concentration on a missing effect should be rejected");

        Ok(())
    }

    #[test]
    fn rejects_invalid_character_uuid() -> TestResult {
        let mut saved = encoded_value(&character_with_complete_runtime_state()?)?;
        *field_mut(&mut saved, "/character/id") = json!("not-a-uuid");

        decode_value(&saved).expect_err("invalid character UUID should be rejected");

        Ok(())
    }

    #[test]
    fn rejects_invalid_ruleset_version() -> TestResult {
        let mut saved = encoded_value(&character_with_complete_runtime_state()?)?;
        *field_mut(&mut saved, "/character/ruleset/version") = json!("not-a-version");

        decode_value(&saved).expect_err("invalid ruleset version should be rejected");

        Ok(())
    }

    #[test]
    fn rejects_unknown_duration_kind() -> TestResult {
        let mut saved = encoded_value(&character_with_complete_runtime_state()?)?;
        *field_mut(&mut saved, "/character/state/effects/0/duration/kind") = json!("turns");

        decode_value(&saved).expect_err("unknown duration kind should be rejected");

        Ok(())
    }

    #[test]
    fn rejects_unknown_application_owned_state_field() -> TestResult {
        let mut saved = encoded_value(&character_with_complete_runtime_state()?)?;
        let state = field_mut(&mut saved, "/character/state")
            .as_object_mut()
            .expect("state should be an object");
        state.insert(String::from("unknown"), json!(true));

        decode_value(&saved).expect_err("unknown application-owned state field should be rejected");

        Ok(())
    }

    #[test]
    fn rejects_future_version_before_reading_its_payload() -> TestResult {
        let mut saved = encoded_value(&character_with_complete_runtime_state()?)?;
        *field_mut(&mut saved, "/format_version") = json!(2);
        *field_mut(&mut saved, "/character") = json!({ "unfamiliar": true });

        let error = decode_value(&saved).expect_err("future format version should be rejected");

        assert!(matches!(
            error,
            PersistenceError::UnsupportedVersion { version: 2 }
        ));

        Ok(())
    }

    #[test]
    fn reconstructs_character_with_unavailable_ruleset() -> TestResult {
        let mut saved = encoded_value(&character_with_complete_runtime_state()?)?;
        *field_mut(&mut saved, "/character/ruleset/family") = json!("org.example.unavailable");
        *field_mut(&mut saved, "/character/ruleset/version") = json!("9.8.7");

        let restored = decode_value(&saved)?;
        let expected_ruleset = RulesetId::new("org.example.unavailable", "9.8.7")?;

        assert_eq!(restored.ruleset_id(), &expected_ruleset);

        Ok(())
    }

    #[test]
    fn preserves_nested_opaque_ruleset_json() -> TestResult {
        let original = character_with_complete_runtime_state()?;
        let expected_data = original.ruleset_data().data().clone();
        let restored = decode_character(&encode_character(&original)?)?;

        assert_eq!(restored.ruleset_data().data(), &expected_data);

        Ok(())
    }
}
