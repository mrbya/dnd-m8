use std::collections::{hash_map::Entry, HashMap};

use dnd_m8_ruleset::RulesetRegistry;

use crate::{
    Character, CharacterId, CharacterSummary, CoreError, CoreResult, DamageOutcome, HealingOutcome,
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
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use dnd_m8_ruleset::{RulesetError, RulesetId, RulesetRegistry};

    use crate::{
        test::{
            expected_summary, test_character, test_character_id, TestRuleset,
            INVALID_SCHEMA_MESSAGE, TEST_SCHEMA,
        },
        CharacterService, CoreError,
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
}
