use crate::CharacterService;

/// Root state for the D7D Mate app core.
pub struct AppState {
    /// In-memory character service.
    characters: CharacterService,
}

impl AppState {
    /// Creates application state backed by the provided character service.
    #[must_use]
    pub const fn new(characters: CharacterService) -> Self {
        Self { characters }
    }

    /// Returns the character service.
    #[must_use]
    pub const fn characters(&self) -> &CharacterService {
        &self.characters
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error as StdError;

    use dnd_m8_ruleset::RulesetRegistry;
    use pretty_assertions::assert_eq;

    use crate::{
        test::{expected_summary, test_character, test_character_id, TestRuleset, TEST_SCHEMA},
        AppState, CharacterService,
    };

    /// Result used by fallible unit tests.
    type TestResult = Result<(), Box<dyn StdError>>;

    #[test]
    fn app_state_exposes_configured_character_service() -> TestResult {
        let ruleset = TestRuleset::new()?;
        let ruleset_id = ruleset.id().clone();

        let mut registry = RulesetRegistry::new();
        registry.register(ruleset)?;

        let character_id = test_character_id();
        let mut service = CharacterService::new(registry);

        service.add_character(test_character(character_id, ruleset_id, TEST_SCHEMA))?;

        let state = AppState::new(service);
        let summary = state.characters().summarize_character(character_id)?;

        assert_eq!(summary, expected_summary());

        Ok(())
    }
}
