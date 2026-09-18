use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

use crate::{
    CharacterSummary, RulesetCharacterData, RulesetError, RulesetId, RulesetMetadata, RulesetResult,
};

/// D7D Mate contract implemented by a concrete ruleset.
pub trait Ruleset: Send + Sync {
    /// Returns stable metadata for this implementation.
    fn metadata(&self) -> &RulesetMetadata;

    /// Projects opaque character data into the common summary.
    ///
    /// # Errors
    ///
    /// Returns [`RulesetError`] when the schema or payload cannot be
    /// interpreted by this ruleset.
    fn sumarize_character(
        &self,
        character: &RulesetCharacterData,
    ) -> RulesetResult<CharacterSummary>;
}

/// Concrete rulesets available to an app instance.
#[derive(Default)]
pub struct RulesetRegistry {
    /// Available rulesets.
    rulesets: HashMap<RulesetId, Arc<dyn Ruleset>>,
}

impl RulesetRegistry {
    /// Constructs a new, empty ruleset registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers one ruleset implementation.
    ///
    /// # Errors
    ///
    /// Returns [`RulesetError::DuplicateRuleset`] if the exact identifier is already
    /// registered.
    pub fn register<R>(&mut self, ruleset: R) -> RulesetResult<()>
    where
        R: Ruleset + 'static,
    {
        let id = ruleset.metadata().id.clone();

        match self.rulesets.entry(id.clone()) {
            Entry::Occupied(_) => Err(RulesetError::DuplicateRuleset { id }),
            Entry::Vacant(entry) => {
                entry.insert(Arc::new(ruleset));
                Ok(())
            }
        }
    }

    /// Returns the implementation for an exact ruleset identifier.
    pub fn get(&self, id: &RulesetId) -> Option<&dyn Ruleset> {
        self.rulesets.get(id).map(Arc::as_ref)
    }

    /// Returns the number of registered implementations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rulesets.len()
    }

    /// Returns whether no implementations are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rulesets.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::{CharacterSummary, Ruleset, RulesetId, RulesetMetadata, RulesetRegistry};

    #[derive(Debug)]
    struct TestRuleset {
        metadata: RulesetMetadata,
        summary: CharacterSummary,
    }

    impl TestRuleset {
        pub fn new() -> Self {
            Self {
                metadata: RulesetMetadata {
                    id: RulesetId::new("org.dnd-m8.test", "1.0.0").expect("valid id"),
                    display_name: String::from("test ruleset"),
                    description: String::from("kung foo"),
                },
                summary: CharacterSummary {
                    name: String::from("bruce lee"),
                    level: 19,
                    class_name: String::from("monk"),
                    current_hit_points: 1,
                    max_hit_points: 99,
                    armor_class: 18,
                    proficiency_bonus: 5,
                    speed: 30,
                },
            }
        }
    }

    impl Ruleset for TestRuleset {
        fn metadata(&self) -> &crate::RulesetMetadata {
            &self.metadata
        }

        fn sumarize_character(
            &self,
            _: &crate::RulesetCharacterData,
        ) -> crate::RulesetResult<CharacterSummary> {
            Ok(self.summary.clone())
        }
    }

    #[test]
    fn registry_rejects_duplicate_exact_id() -> Result<(), ()> {
        let mut registry = RulesetRegistry::new();

        registry
            .register(TestRuleset::new())
            .expect("first registration");

        let error = registry
            .register(TestRuleset::new())
            .expect_err("duplicate registration must fail");

        match error {
            crate::RulesetError::DuplicateRuleset { id } => {
                assert_eq!(
                    id,
                    RulesetId::new("org.dnd-m8.test", "1.0.0").expect("valid id")
                );
                Ok(())
            }
            _ => Err(()),
        }
    }
}
