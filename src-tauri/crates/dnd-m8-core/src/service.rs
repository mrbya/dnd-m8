use std::collections::{hash_map::Entry, HashMap};

use dnd_m8_ruleset::{CharacterSummary, RulesetRegistry};

use crate::{Character, CharacterId, CoreError, CoreResult};

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

    /// Produces the ruleset-derived summary of a character.
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

        ruleset
            .summarize_character(character.ruleset_data())
            .map_err(|source| {
                Box::new(CoreError::Ruleset {
                    character_id,
                    ruleset_id: ruleset_id.clone(),
                    source,
                })
            })
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
        ))?;

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
        ))?;

        let result = service.add_character(test_character(character_id, ruleset_id, TEST_SCHEMA));

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
    fn summarize_character_delegates_to_associated_ruleset() -> TestResult {
        let (mut service, ruleset_id) = service_with_ruleset()?;
        let character_id = test_character_id();

        service.add_character(test_character(character_id, ruleset_id, TEST_SCHEMA))?;

        let summary = service.summarize_character(character_id)?;

        assert_eq!(summary, expected_summary());

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
        ))?;

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
        ))?;

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
}
