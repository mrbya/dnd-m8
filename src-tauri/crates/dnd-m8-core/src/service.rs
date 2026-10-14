use std::collections::{hash_map::Entry, HashMap};

use dnd_m8_ruleset::RulesetRegistry;

use crate::{
    character::{
        EffectAdvanceOutcome, EffectApplyOutcome, EffectId, EffectInstanceId, EffectRemoveOutcome,
    },
    Character, CharacterId, CharacterSummary, ConcentrationChangeOutcome, ConditionApplyOutcome,
    ConditionId, ConditionInstanceId, ConditionRemoveOutcome, CoreError, CoreResult, DamageOutcome,
    EffectDuration, HealingOutcome, ResourceId, ResourceRestoreOutcome, ResourceSpendOutcome,
};

/// Provides application operations to manage characters.
pub struct CharacterService {
    /// Registry containing available rulesets.
    rulesets: RulesetRegistry,
    /// In-memory map of managed characters.
    characters: HashMap<CharacterId, Character>,
}

impl CharacterService {
    /// Creates an empty character service using the provided rulesets.
    #[must_use]
    pub fn new(rulesets: RulesetRegistry) -> Self {
        Self {
            rulesets,
            characters: HashMap::new(),
        }
    }

    /// Registers a character with the service.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::DuplicateCharacter`] if a character with the same
    /// identifier is already registered.
    pub fn add_character(&mut self, character: Character) -> CoreResult<()> {
        let id = character.id();

        match self.characters.entry(id) {
            Entry::Occupied(_) => Err(Box::new(CoreError::DuplicateCharacter { id })),
            Entry::Vacant(entry) => {
                entry.insert(character);
                Ok(())
            }
        }
    }

    /// Finds a character by identifier.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::CharacterNotFound`] when the character is unavailable.
    pub fn character(&self, character_id: CharacterId) -> CoreResult<&Character> {
        self.characters
            .get(&character_id)
            .ok_or(Box::new(CoreError::CharacterNotFound { id: character_id }))
    }

    /// Finds a mutable character by identifier.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::CharacterNotFound`] when the character is unavailable.
    fn character_mut(&mut self, character_id: CharacterId) -> CoreResult<&mut Character> {
        self.characters
            .get_mut(&character_id)
            .ok_or(Box::new(CoreError::CharacterNotFound { id: character_id }))
    }

    /// Produces the combined summary of a character.
    ///
    /// # Errors
    ///
    /// Returns an error when the character or its ruleset cannot be found,
    /// or when its ruleset cannot interpret the stored data.
    pub fn summarize_character(&self, character_id: CharacterId) -> CoreResult<CharacterSummary> {
        let character = self.character(character_id)?;
        let ruleset_id = character.ruleset_id();

        let ruleset = self.rulesets.get(ruleset_id).ok_or_else(|| {
            Box::new(CoreError::RulesetNotFound {
                character_id,
                ruleset_id: ruleset_id.clone(),
            })
        })?;

        let ruleset_summary = ruleset
            .summarize_character(character.ruleset_data())
            .map_err(|source| {
                Box::new(CoreError::Ruleset {
                    character_id,
                    ruleset_id: ruleset_id.clone(),
                    source,
                })
            })?;

        Ok(CharacterSummary::from_ruleset(
            ruleset_summary,
            character.state().hit_points(),
        ))
    }

    /// Applies damage to a stored character.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::CharacterNotFound`] when the character is unavailable.
    pub fn take_damage(
        &mut self,
        character_id: CharacterId,
        amount: u32,
    ) -> CoreResult<DamageOutcome> {
        Ok(self.character_mut(character_id)?.take_damage(amount))
    }

    /// Restores a stored character's ordinary hit points.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::CharacterNotFound`] when the character is unavailable.
    pub fn heal(&mut self, character_id: CharacterId, amount: u32) -> CoreResult<HealingOutcome> {
        Ok(self.character_mut(character_id)?.heal(amount))
    }

    /// Replaces a stored character's temporary hit points.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::CharacterNotFound`] when the character is unavailable.
    pub fn replace_temporary_hit_points(
        &mut self,
        character_id: CharacterId,
        amount: u32,
    ) -> CoreResult<u32> {
        Ok(self
            .character_mut(character_id)?
            .replace_temporary_hit_points(amount))
    }

    /// Spends a resource belonging to a managed character.
    ///
    /// # Errors
    ///
    /// Returns an error when the character or resource is unavailable, or when
    /// the resource does not contain the requested amount.
    pub fn spend_resource(
        &mut self,
        character_id: CharacterId,
        resource_id: &ResourceId,
        amount: u32,
    ) -> CoreResult<ResourceSpendOutcome> {
        self.character_mut(character_id)?
            .spend_resource(resource_id, amount)
    }

    /// Restores a resource belonging to a managed character.
    ///
    /// Restoration behavior such as short-rest or long-rest recovery is determined
    /// by the ruleset; this operation only applies an explicit amount.
    ///
    /// # Errors
    ///
    /// Returns an error when the character or resource is unavailable.
    pub fn restore_resource(
        &mut self,
        character_id: CharacterId,
        resource_id: &ResourceId,
        amount: u32,
    ) -> CoreResult<ResourceRestoreOutcome> {
        self.character_mut(character_id)?
            .restore_resource(resource_id, amount)
    }

    /// Applies a condition to a managed character.
    ///
    /// # Errors
    ///
    /// Returns an error when the character is unavailable or the condition
    /// instance cannot be stored.
    pub fn apply_condition(
        &mut self,
        character_id: CharacterId,
        condition_id: ConditionId,
    ) -> CoreResult<ConditionApplyOutcome> {
        self.character_mut(character_id)?
            .apply_condition(condition_id)
    }

    /// Removes one condition instance from a managed character.
    ///
    /// # Errors
    ///
    /// Returns an error when the character or condition instance is
    /// unavailable.
    pub fn remove_condition(
        &mut self,
        character_id: CharacterId,
        instance_id: ConditionInstanceId,
    ) -> CoreResult<ConditionRemoveOutcome> {
        self.character_mut(character_id)?
            .remove_condition(instance_id)
    }

    //// Applies an effect to a managed character.
    ///
    /// # Errors
    ///
    /// Returns an error when the character is unavailable or the effect
    /// instance cannot be stored.
    pub fn apply_effect(
        &mut self,
        character_id: CharacterId,
        effect_id: EffectId,
        duration: EffectDuration,
    ) -> CoreResult<EffectApplyOutcome> {
        self.character_mut(character_id)?
            .apply_effect(effect_id, duration)
    }

    /// Removes one effect instance from a managed character.
    ///
    /// Removing the effect sustained through concentration also clears that
    /// character's concentration.
    ///
    /// # Errors
    ///
    /// Returns an error when the character or the effect instance is
    /// unavailable.
    pub fn remove_effect(
        &mut self,
        character_id: CharacterId,
        instance_id: EffectInstanceId,
    ) -> CoreResult<EffectRemoveOutcome> {
        self.character_mut(character_id)?.remove_effect(instance_id)
    }

    /// Advances the duration of one effect applied to a managed character.
    ///
    /// Expiring the effect sustained through concentration also clears that
    /// character's concentration.
    ///
    /// # Errors
    ///
    /// Returns an error when the character or the effect instance is
    /// unavailable.
    pub fn advance_effect_rounds(
        &mut self,
        character_id: CharacterId,
        instance_id: EffectInstanceId,
        rounds: u32,
    ) -> CoreResult<EffectAdvanceOutcome> {
        self.character_mut(character_id)?
            .advance_effect_rounds(instance_id, rounds)
    }

    /// Starts concentration on one of a managed character's active effects.
    ///
    /// # Errors
    ///
    /// Returns an error when the character or effect is unavailable, or
    /// another effect is already sustained through concentration.
    pub fn start_concentration(
        &mut self,
        character_id: CharacterId,
        instance_id: EffectInstanceId,
    ) -> CoreResult<ConcentrationChangeOutcome> {
        self.character_mut(character_id)?
            .start_concentration(instance_id)
    }

    /// Replaces a managed character's concentration.
    ///
    /// # Errors
    ///
    /// Returns an error when the character, target effect, or previously
    /// concentrated effect is unavailable.
    pub fn replace_concentration(
        &mut self,
        character_id: CharacterId,
        instance_id: EffectInstanceId,
    ) -> CoreResult<ConcentrationChangeOutcome> {
        self.character_mut(character_id)?
            .replace_concentration(instance_id)
    }

    /// Ends a managed character's concentration and removes its effect.
    ///
    /// # Errors
    ///
    /// Returns an error when the character or concentrated effect is
    /// unavailable.
    pub fn end_concentration(
        &mut self,
        character_id: CharacterId,
    ) -> CoreResult<ConcentrationChangeOutcome> {
        self.character_mut(character_id)?.end_concentration()
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use dnd_m8_ruleset::{RulesetError, RulesetId, RulesetRegistry};

    use crate::{
        test::{
            expected_summary, test_character, test_character_id, test_character_with_effects,
            test_character_with_resources, test_character_with_runtime_state, test_effect_duration,
            test_effect_instance_id, test_hit_points, test_resource_id, TestRuleset,
            INVALID_SCHEMA_MESSAGE, TEST_SCHEMA,
        },
        CharacterId, CharacterService, ConditionId, ConditionInstanceId, CoreError, EffectDuration,
        EffectId, EffectInstanceId, ResourceId,
    };

    type TestResult<T = ()> = Result<T, Box<dyn Error>>;

    fn service_with_ruleset() -> TestResult<(CharacterService, RulesetId)> {
        let ruleset = TestRuleset::new()?;
        let ruleset_id = ruleset.id().clone();

        let mut registry = RulesetRegistry::new();
        registry.register(ruleset)?;

        Ok((CharacterService::new(registry), ruleset_id))
    }

    #[test]
    fn character_returns_registered_character() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();

        service.add_character(test_character(
            character_id,
            ruleset_id.clone(),
            TEST_SCHEMA,
        )?)?;

        let character = service.character(character_id)?;

        assert_eq!(character.id(), character_id);
        assert_eq!(character.ruleset_id(), &ruleset_id);
        assert_eq!(character.ruleset_data().schema(), TEST_SCHEMA);

        Ok(())
    }

    #[test]
    fn character_rejects_unknown_identifier() {
        let service = CharacterService::new(RulesetRegistry::new());
        let character_id = test_character_id();

        let result = service.character(character_id);

        assert!(matches!(
            result,
            Err(error)
                if matches!(
                    error.as_ref(),
                    CoreError::CharacterNotFound { id }
                        if *id == character_id
                )
        ));
    }

    #[test]
    fn add_character_rejects_duplicate_identifier() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();

        service.add_character(test_character(
            character_id,
            ruleset_id.clone(),
            TEST_SCHEMA,
        )?)?;

        let result = service.add_character(test_character(character_id, ruleset_id, TEST_SCHEMA)?);

        assert!(matches!(
            result,
            Err(error)
                if matches!(
                    error.as_ref(),
                    CoreError::DuplicateCharacter { id }
                        if *id == character_id
                )
        ));

        Ok(())
    }

    #[test]
    fn summarize_character_combines_ruleset_projection_with_runtime_hit_points() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();

        service.add_character(test_character(character_id, ruleset_id, TEST_SCHEMA)?)?;

        let summary = service.summarize_character(character_id)?;

        assert_eq!(summary, expected_summary()?);

        Ok(())
    }

    #[test]
    fn summarize_character_rejects_unregistered_ruleset() -> TestResult {
        let ruleset = TestRuleset::new()?;
        let ruleset_id = ruleset.id().clone();
        let character_id = test_character_id();

        let mut service = CharacterService::new(RulesetRegistry::new());
        service.add_character(test_character(
            character_id,
            ruleset_id.clone(),
            TEST_SCHEMA,
        )?)?;

        let result = service.summarize_character(character_id);

        assert!(matches!(
            result,
            Err(error)
                if matches!(
                    error.as_ref(),
                    CoreError::RulesetNotFound {
                        character_id: actual_character_id,
                        ruleset_id: actual_ruleset_id,
                    } if *actual_character_id == character_id
                        && actual_ruleset_id == &ruleset_id
                )
        ));

        Ok(())
    }

    #[test]
    fn summarize_character_preserves_ruleset_error_context() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();

        service.add_character(test_character(
            character_id,
            ruleset_id.clone(),
            "org.dnd-m8.test.unsupported@1",
        )?)?;

        let result = service.summarize_character(character_id);

        assert!(matches!(
            result,
            Err(error)
                if matches!(
                    error.as_ref(),
                    CoreError::Ruleset {
                        character_id: actual_character_id,
                        ruleset_id: actual_ruleset_id,
                        source: RulesetError::Generic { msg },
                    } if *actual_character_id == character_id
                        && actual_ruleset_id == &ruleset_id
                        && msg == INVALID_SCHEMA_MESSAGE
                )
        ));

        Ok(())
    }

    #[test]
    fn damage_mutates_stored_character_and_consumes_temporary_hit_points() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character(character_id, ruleset_id, TEST_SCHEMA)?)?;

        let outcome = service.take_damage(character_id, 8)?;
        let hit_points = service.character(character_id)?.state().hit_points();

        assert_eq!(outcome.temporary_lost, 7);
        assert_eq!(outcome.hit_points_lost, 1);
        assert_eq!(hit_points.current(), 0);
        assert_eq!(hit_points.temporary(), 0);

        Ok(())
    }

    #[test]
    fn healing_mutates_stored_character_without_changing_temporary_hit_points() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character(character_id, ruleset_id, TEST_SCHEMA)?)?;

        let outcome = service.heal(character_id, 4)?;
        let hit_points = service.character(character_id)?.state().hit_points();

        assert_eq!(outcome.restored, 4);
        assert_eq!(hit_points.current(), 5);
        assert_eq!(hit_points.temporary(), 7);

        Ok(())
    }

    #[test]
    fn healing_is_capped_at_maximum_hit_points() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character(character_id, ruleset_id, TEST_SCHEMA)?)?;

        let outcome = service.heal(character_id, 100)?;

        assert_eq!(outcome.restored, 98);
        assert_eq!(outcome.excess, 2);
        assert_eq!(
            service
                .character(character_id)?
                .state()
                .hit_points()
                .current(),
            99
        );

        Ok(())
    }

    #[test]
    fn replacing_temporary_hit_points_persists_the_new_value() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character(character_id, ruleset_id, TEST_SCHEMA)?)?;

        let previous = service.replace_temporary_hit_points(character_id, 12)?;

        assert_eq!(previous, 7);
        assert_eq!(
            service
                .character(character_id)?
                .state()
                .hit_points()
                .temporary(),
            12
        );

        Ok(())
    }

    #[test]
    fn operations_reject_unknown_character_identifier() {
        let mut service = CharacterService::new(RulesetRegistry::new());
        let character_id = test_character_id();

        for result in [
            service.take_damage(character_id, 1).map(|_| ()),
            service.heal(character_id, 1).map(|_| ()),
            service
                .replace_temporary_hit_points(character_id, 1)
                .map(|_| ()),
        ] {
            assert!(matches!(
                result,
                Err(error)
                    if matches!(
                        error.as_ref(),
                        CoreError::CharacterNotFound { id } if *id == character_id
                    )
            ));
        }
    }

    #[test]
    fn spending_restoring_resource_mutates_character() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character_with_resources(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;
        let resource_id = test_resource_id()?;

        let spend_result = service.spend_resource(character_id, &resource_id, 4)?;

        assert_eq!(spend_result.spent, 4);
        assert_eq!(spend_result.previous, 6);
        assert_eq!(spend_result.current, 2);

        let restore_result = service.restore_resource(character_id, &resource_id, 3)?;

        assert_eq!(restore_result.restored, 3);
        assert_eq!(restore_result.previous, 2);
        assert_eq!(restore_result.current, 5);

        let character = service.character(character_id)?;
        let resource = character
            .state()
            .resources()
            .resource(&resource_id)
            .ok_or("fixture resource disappeared")?;

        assert_eq!(resource.current(), 5);

        Ok(())
    }

    #[test]
    fn overspending_resources() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character_with_resources(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;
        let resource_id = test_resource_id()?;

        let result = service.spend_resource(character_id, &resource_id, 8);

        assert!(matches!(
            result,
            Err(error)
                if matches!(
                    error.as_ref(),
                    CoreError::InsufficientResource {
                        requested: 8,
                        available: 6,
                        ..
                    }
                )
        ));

        let character = service.character(character_id)?;
        let resource = character
            .state()
            .resources()
            .resource(&resource_id)
            .ok_or("fixture resource disappeared")?;

        assert_eq!(resource.current(), 6);

        Ok(())
    }

    #[test]
    fn resource_operations_reject_unknown_character() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character_with_resources(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;
        let resource_id = test_resource_id()?;

        service
            .spend_resource(CharacterId::new(), &resource_id, 9)
            .expect_err("resource operation should reject unknown character");

        service
            .restore_resource(CharacterId::new(), &resource_id, 9)
            .expect_err("resource operation should reject unknown character");

        Ok(())
    }

    #[test]
    fn resource_operations_reject_unknown_resource() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character_with_resources(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;
        let resource_id = ResourceId::new("unknown")?;

        service
            .spend_resource(character_id, &resource_id, 9)
            .expect_err("resource operation should reject unknown resource");

        service
            .restore_resource(character_id, &resource_id, 9)
            .expect_err("resource operation should reject unknown resource");

        Ok(())
    }

    #[test]
    fn condition_operations_mutate_character() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character(character_id, ruleset_id, TEST_SCHEMA)?)?;

        let result = service.apply_condition(character_id, ConditionId::new("condition.1")?)?;
        assert_eq!(result.condition_id, ConditionId::new("condition.1")?);

        assert_eq!(
            service.character(character_id)?.state().conditions().len(),
            1
        );

        Ok(())
    }

    #[test]
    fn condition_operations_reject_unknown_characters() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character(character_id, ruleset_id, TEST_SCHEMA)?)?;
        let unknown_id = CharacterId::new();

        service
            .apply_condition(unknown_id, ConditionId::new("condition.1")?)
            .expect_err("operation should reject unknown character");

        Ok(())
    }

    #[test]
    fn removing_unknown_condition_instance_preserves_state() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character_with_resources(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;

        service.apply_condition(character_id, ConditionId::new("condition.1")?)?;
        service
            .remove_condition(character_id, ConditionInstanceId::new())
            .expect_err("should fail on unknown condition instance");

        assert_eq!(
            service
                .character(character_id)?
                .state()
                .hit_points()
                .current(),
            test_hit_points()?.current()
        );

        Ok(())
    }

    #[test]
    fn effect_operations_mutate_character() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character(character_id, ruleset_id, TEST_SCHEMA)?)?;

        let result = service.apply_effect(
            character_id,
            EffectId::new("effect.1")?,
            EffectDuration::UntilRemoved,
        )?;
        assert_eq!(result.effect_id, EffectId::new("effect.1")?);

        assert_eq!(service.character(character_id)?.state().effects().len(), 1);

        Ok(())
    }

    #[test]
    fn effect_operations_reject_unknown_characters() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character(character_id, ruleset_id, TEST_SCHEMA)?)?;
        let unknown_id = CharacterId::new();

        service
            .apply_effect(
                unknown_id,
                EffectId::new("effect.1")?,
                EffectDuration::UntilRemoved,
            )
            .expect_err("operation should reject unknown character");

        let result = service.apply_effect(
            character_id,
            EffectId::new("effect.1")?,
            EffectDuration::rounds(3)?,
        )?;

        let error = service
            .advance_effect_rounds(unknown_id, result.instance_id, 3)
            .expect_err("should reject unknown character");

        assert!(matches!(*error, CoreError::CharacterNotFound { .. }));

        Ok(())
    }

    #[test]
    fn removing_unknown_effect_instance_preserves_state() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character_with_resources(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;

        service.apply_effect(
            character_id,
            EffectId::new("effect.1")?,
            EffectDuration::UntilRemoved,
        )?;
        service
            .remove_effect(character_id, EffectInstanceId::new())
            .expect_err("should fail on unknown effect instance");

        assert_eq!(
            service
                .character(character_id)?
                .state()
                .hit_points()
                .current(),
            test_hit_points()?.current()
        );

        Ok(())
    }

    #[test]
    fn starting_concentration_references_existing_effect() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character_with_runtime_state(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;

        let mut result = service.start_concentration(character_id, test_effect_instance_id())?;

        assert_eq!(result.current, Some(test_effect_instance_id()));

        let before = service.character(character_id)?.state().clone();

        // Restarting concentration on the same instance is a no-op.
        result = service.start_concentration(character_id, test_effect_instance_id())?;

        assert_eq!(result.current, Some(test_effect_instance_id()));
        assert_eq!(result.removed_effect, None);
        assert_eq!(service.character(character_id)?.state(), &before);

        Ok(())
    }

    #[test]
    fn starting_concentration_on_different_instance_fails_and_preserves_state() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character_with_runtime_state(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;

        let result = service.start_concentration(character_id, test_effect_instance_id())?;
        assert_eq!(result.current, Some(test_effect_instance_id()));

        let other = service.apply_effect(
            character_id,
            EffectId::new("effect.other")?,
            EffectDuration::UntilRemoved,
        )?;

        let before = service.character(character_id)?.state().clone();

        service
            .start_concentration(character_id, other.instance_id)
            .expect_err("starting a different concentration should fail");

        assert_eq!(service.character(character_id)?.state(), &before);

        Ok(())
    }

    #[test]
    fn replacing_concentration_removes_previous_instance() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character_with_effects(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;

        let new_id = service
            .apply_effect(
                character_id,
                EffectId::new("effect.test")?,
                EffectDuration::UntilRemoved,
            )?
            .instance_id;

        let mut result = service.start_concentration(character_id, test_effect_instance_id())?;
        assert_eq!(result.current, Some(test_effect_instance_id()));

        result = service.replace_concentration(character_id, new_id)?;
        assert_eq!(result.previous, Some(test_effect_instance_id()));
        assert_eq!(result.current, Some(new_id));
        assert_eq!(
            service
                .character(character_id)
                .expect("should be present")
                .state()
                .effects()
                .effect(test_effect_instance_id()),
            None
        );

        Ok(())
    }

    #[test]
    fn replacing_concentration_with_missing_preserves_previous_state() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character_with_runtime_state(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;

        let result = service.start_concentration(character_id, test_effect_instance_id())?;
        assert_eq!(result.current, Some(test_effect_instance_id()));

        let before = service.character(character_id)?.state().clone();

        service
            .replace_concentration(character_id, EffectInstanceId::new())
            .expect_err("replacing concentration with missing instance should fail");

        assert_eq!(service.character(character_id)?.state(), &before);

        Ok(())
    }

    #[test]
    fn replacing_concentration_on_current_instance_preserves_duration() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character_with_effects(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;

        let mut result = service.start_concentration(character_id, test_effect_instance_id())?;
        assert_eq!(result.current, Some(test_effect_instance_id()));

        let before = service.character(character_id)?.state().clone();
        result = service.replace_concentration(character_id, test_effect_instance_id())?;
        assert_eq!(result.current, Some(test_effect_instance_id()));
        assert_eq!(service.character(character_id)?.state(), &before);

        assert_eq!(
            service
                .character(character_id)
                .expect("char should be present")
                .state()
                .effects()
                .effect(test_effect_instance_id())
                .expect("effect should be present")
                .duration(),
            test_effect_duration()?
        );

        Ok(())
    }

    #[test]
    fn exact_effect_expiration_removes_concentration() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character_with_runtime_state(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;

        let result = service.start_concentration(character_id, test_effect_instance_id())?;
        assert_eq!(result.current, Some(test_effect_instance_id()));

        let advance_result =
            service.advance_effect_rounds(character_id, test_effect_instance_id(), 9)?;
        assert_eq!(advance_result.remaining_duration, None);
        assert_eq!(
            service
                .character(character_id)
                .expect("should be present")
                .state()
                .concentration(),
            None
        );

        Ok(())
    }

    #[test]
    fn ending_concentration_removes_effect_and_can_be_repeated() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();

        service.add_character(test_character_with_runtime_state(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;

        service.start_concentration(character_id, test_effect_instance_id())?;
        service.end_concentration(character_id)?;

        let state = service.character(character_id)?.state();

        assert_eq!(state.concentration(), None);
        assert!(state.effects().effect(test_effect_instance_id()).is_none());
        assert_eq!(state.resources().len(), 1);
        assert_eq!(state.conditions().len(), 1);

        let before = state.clone();

        service.end_concentration(character_id)?;

        assert_eq!(service.character(character_id)?.state(), &before);

        Ok(())
    }

    #[test]
    fn partial_effect_advancement_preserves_concentration() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character_with_effects(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;

        let result = service.start_concentration(character_id, test_effect_instance_id())?;
        assert_eq!(result.current, Some(test_effect_instance_id()));

        let advance_result =
            service.advance_effect_rounds(character_id, test_effect_instance_id(), 3)?;
        assert!(matches!(
            advance_result.remaining_duration,
            Some(EffectDuration::Rounds { .. })
        ));
        assert_eq!(
            service
                .character(character_id)
                .expect("should be present")
                .state()
                .concentration(),
            Some(test_effect_instance_id())
        );

        Ok(())
    }

    #[test]
    fn zero_effect_advancement_preserves_concentration_and_state() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character_with_runtime_state(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;
        service.start_concentration(character_id, test_effect_instance_id())?;
        let before = service.character(character_id)?.state().clone();

        let outcome = service.advance_effect_rounds(character_id, test_effect_instance_id(), 0)?;

        assert_eq!(outcome.elapsed_rounds, 0);
        assert_eq!(service.character(character_id)?.state(), &before);

        Ok(())
    }

    #[test]
    fn overshoot_effect_expiration_removes_concentration() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character_with_effects(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;
        service.start_concentration(character_id, test_effect_instance_id())?;

        let outcome =
            service.advance_effect_rounds(character_id, test_effect_instance_id(), u32::MAX)?;
        let state = service.character(character_id)?.state();

        assert_eq!(outcome.remaining_duration, None);
        assert_eq!(state.concentration(), None);
        assert!(state.effects().effect(test_effect_instance_id()).is_none());

        Ok(())
    }

    #[test]
    fn removing_concentrated_effect_clears_concentration() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character_with_runtime_state(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;
        service.start_concentration(character_id, test_effect_instance_id())?;

        service.remove_effect(character_id, test_effect_instance_id())?;

        let state = service.character(character_id)?.state();
        assert_eq!(state.concentration(), None);
        assert!(state.effects().effect(test_effect_instance_id()).is_none());
        assert_eq!(state.resources().len(), 1);
        assert_eq!(state.conditions().len(), 1);

        Ok(())
    }

    #[test]
    fn unrelated_effect_removal_and_expiration_preserve_concentration() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character_with_runtime_state(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;
        service.start_concentration(character_id, test_effect_instance_id())?;
        let removable = service
            .apply_effect(
                character_id,
                EffectId::new("effect.removable")?,
                EffectDuration::UntilRemoved,
            )?
            .instance_id;
        let expiring = service
            .apply_effect(
                character_id,
                EffectId::new("effect.expiring")?,
                EffectDuration::rounds(1)?,
            )?
            .instance_id;

        service.remove_effect(character_id, removable)?;
        service.advance_effect_rounds(character_id, expiring, 1)?;

        let state = service.character(character_id)?.state();
        assert_eq!(state.concentration(), Some(test_effect_instance_id()));
        assert!(state.effects().effect(test_effect_instance_id()).is_some());
        assert!(state.effects().effect(removable).is_none());
        assert!(state.effects().effect(expiring).is_none());

        Ok(())
    }

    #[test]
    fn replacing_absent_concentration_starts_without_removing_effects() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character_with_runtime_state(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;
        let target = service
            .apply_effect(
                character_id,
                EffectId::new("effect.target")?,
                EffectDuration::UntilRemoved,
            )?
            .instance_id;

        let outcome = service.replace_concentration(character_id, target)?;
        let state = service.character(character_id)?.state();

        assert_eq!(outcome.previous, None);
        assert_eq!(outcome.current, Some(target));
        assert_eq!(outcome.removed_effect, None);
        assert_eq!(state.concentration(), Some(target));
        assert!(state.effects().effect(test_effect_instance_id()).is_some());
        assert!(state.effects().effect(target).is_some());

        Ok(())
    }

    #[test]
    fn same_effect_instances_remain_independent_through_service() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character(character_id, ruleset_id, TEST_SCHEMA)?)?;
        let effect_id = EffectId::new("effect.repeated")?;
        let first = service
            .apply_effect(character_id, effect_id.clone(), EffectDuration::rounds(2)?)?
            .instance_id;
        let second = service
            .apply_effect(character_id, effect_id.clone(), EffectDuration::rounds(4)?)?
            .instance_id;

        service.advance_effect_rounds(character_id, first, 2)?;
        let effects = service.character(character_id)?.state().effects();

        assert!(effects.effect(first).is_none());
        assert_eq!(effects.instance_count(&effect_id), 1);
        assert_eq!(
            effects
                .effect(second)
                .expect("second instance should remain")
                .duration(),
            EffectDuration::rounds(4)?,
        );

        Ok(())
    }

    #[test]
    fn concentration_on_effect_belonging_to_another_character_is_rejected() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id1 = test_character_id();
        let character_id2 = CharacterId::new();
        service.add_character(test_character_with_runtime_state(
            character_id1,
            ruleset_id.clone(),
            TEST_SCHEMA,
        )?)?;
        service.add_character(test_character_with_runtime_state(
            character_id2,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;
        let id = service
            .apply_effect(
                character_id2,
                EffectId::new("effect.test")?,
                EffectDuration::UntilRemoved,
            )?
            .instance_id;

        let before_first = service.character(character_id1)?.state().clone();
        let before_second = service.character(character_id2)?.state().clone();

        service
            .start_concentration(character_id1, id)
            .expect_err("concentration on effect instance not belonging to character should fail");

        assert_eq!(service.character(character_id1)?.state(), &before_first);
        assert_eq!(service.character(character_id2)?.state(), &before_second);

        Ok(())
    }

    #[test]
    fn concentration_operations_reject_unknown_character_without_mutating_managed_character(
    ) -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();
        service.add_character(test_character_with_runtime_state(
            character_id,
            ruleset_id,
            TEST_SCHEMA,
        )?)?;
        let before = service.character(character_id)?.state().clone();
        let unknown_character_id = CharacterId::new();

        service
            .start_concentration(unknown_character_id, test_effect_instance_id())
            .expect_err("unknown character should be rejected when starting concentration");
        service
            .replace_concentration(unknown_character_id, test_effect_instance_id())
            .expect_err("unknown character should be rejected when replacing concentration");
        service
            .end_concentration(unknown_character_id)
            .expect_err("unknown character should be rejected when ending concentration");

        assert_eq!(service.character(character_id)?.state(), &before);

        Ok(())
    }
}
