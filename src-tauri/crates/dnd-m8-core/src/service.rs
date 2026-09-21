use std::collections::HashMap;

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

    /// Finds a character by identifier.
    ///
    /// # Errors
    ///
    /// Returns [`Error::CharacterNotFound`] when the character is unavailable.
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
