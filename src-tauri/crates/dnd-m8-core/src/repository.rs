//! Character storage contracts and an in-memory repository.
//!
//! Repositories store complete character snapshots without interpreting
//! ruleset-owned definitions. Loaded characters are independently owned;
//! mutations become stored state only through an explicit update.

use std::collections::{btree_map::Entry, BTreeMap};

use crate::{
    decode_character, encode_character, Character, CharacterId, PersistenceError, PersistenceResult,
};

/// Storage boundary for complete character snapshot.
///
/// Failed create, update, and delete operations perserve existing records.
/// Implementations do not require the character's ruleset to be installed.
///
/// This interface is synchronous. Durable implementations must be called outside the UI thread
/// when their operations may block.
pub trait CharacterRepository {
    /// Stores a new character without replacing an existing record.
    ///
    /// # Errors
    ///
    /// Returns an error when the identifier already exists, encoding fails
    /// or the storage backend cannot complete the operation.
    fn create(&mut self, character: &Character) -> PersistenceResult<()>;

    /// Loads an independently owned character.
    ///
    /// # Errors
    ///
    /// Returns an error when the identifier is missing from the repositoru, the saved
    /// record cannot be reconstructe, or the storage cannot be read.
    fn load(&self, id: CharacterId) -> PersistenceResult<Character>;

    /// Replaces the complete snapshot of an already stored character.
    ///
    /// # Errors
    ///
    /// Returns an error when the identifier is missing from the repository,
    /// encoding fails, or the storage backend cannot complete the operation.
    fn update(&mut self, character: &Character) -> PersistenceResult<()>;

    /// Lists stored character identifiers in an ascending order.
    ///
    /// Listing does not interpret ruleset-owned character definitions.
    ///
    /// # Errors
    ///
    /// Returns an error when the storage cannot be read or enumerated.
    fn list(&self) -> PersistenceResult<Vec<CharacterId>>;

    /// Delets an existing character record.
    ///
    /// # Errors
    ///
    /// Returns an error when the identifier is missing from the repository or
    /// the storage backend cannot complete the operation.
    fn delete(&mut self, id: CharacterId) -> PersistenceResult<()>;
}

/// Transient repository storing versioned character snapshots.
///
/// This implementation establishes repository behavior for tests and
/// development. Its records do not survive application restarts.
#[derive(Debug, Default)]
pub struct InMemoryCharacterRepository {
    /// Encoded snapshots indexed by persistent character identity.
    records: BTreeMap<CharacterId, String>,
}

impl InMemoryCharacterRepository {
    /// Constructs an empty in-memory repository.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl CharacterRepository for InMemoryCharacterRepository {
    fn create(&mut self, character: &Character) -> PersistenceResult<()> {
        let id = character.id();

        match self.records.entry(id) {
            Entry::Occupied(_) => Err(PersistenceError::CharacterAlreadyExists { id }),
            Entry::Vacant(entry) => {
                let encoded = encode_character(character)?;

                entry.insert(encoded);

                Ok(())
            }
        }
    }

    fn load(&self, id: CharacterId) -> PersistenceResult<Character> {
        let encoded = self
            .records
            .get(&id)
            .ok_or(PersistenceError::CharacterNotFound { id })?;

        decode_character(encoded)
    }

    fn update(&mut self, character: &Character) -> PersistenceResult<()> {
        let id = character.id();
        let stored = self
            .records
            .get_mut(&id)
            .ok_or(PersistenceError::CharacterNotFound { id })?;

        let encoded = encode_character(character)?;

        *stored = encoded;

        Ok(())
    }

    fn list(&self) -> PersistenceResult<Vec<CharacterId>> {
        Ok(self.records.keys().copied().collect())
    }

    fn delete(&mut self, id: CharacterId) -> PersistenceResult<()> {
        self.records
            .remove(&id)
            .ok_or(PersistenceError::CharacterNotFound { id })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use dnd_m8_ruleset::{RulesetId, RulesetRegistry};
    use uuid::Uuid;

    use crate::{
        test::{
            test_character, test_character_id, test_character_with_resources, TestRuleset,
            TEST_SCHEMA,
        },
        CharacterId, CharacterRepository, InMemoryCharacterRepository, PersistenceError,
    };

    type TestResult<T = ()> = Result<T, Box<dyn Error>>;

    fn repository_witn_ruleset() -> TestResult<(InMemoryCharacterRepository, RulesetId)> {
        let ruleset = TestRuleset::new()?;
        let ruleset_id = ruleset.id().clone();

        let mut registry = RulesetRegistry::new();
        registry.register(ruleset)?;

        Ok((InMemoryCharacterRepository::new(), ruleset_id))
    }

    #[test]
    fn repository_is_created_and_character_stored() -> TestResult {
        let (mut repository, ruleset_id) = repository_witn_ruleset()?;
        let character = test_character(test_character_id(), ruleset_id, TEST_SCHEMA)?;

        repository.create(&character)?;

        assert_eq!(repository.list().iter().len(), 1);

        Ok(())
    }

    #[test]
    fn duplicate_store_perserves_original_snapshot() -> TestResult {
        let (mut repository, ruleset_id) = repository_witn_ruleset()?;
        let character_id = test_character_id();
        let original =
            test_character_with_resources(character_id, ruleset_id.clone(), TEST_SCHEMA)?;
        repository.create(&original)?;

        let result = repository.create(&test_character(character_id, ruleset_id, TEST_SCHEMA)?);

        assert!(matches!(
            result,
            Err(PersistenceError::CharacterAlreadyExists { id }) if id == character_id
        ));

        let stored = repository.load(character_id)?;
        assert!(!stored.state().resources().is_empty());
        assert_eq!(stored.state().hit_points(), original.state().hit_points());

        Ok(())
    }

    #[test]
    fn mutating_character_does_not_affect_stored_snapshot() -> TestResult {
        let (mut repository, ruleset_id) = repository_witn_ruleset()?;
        let character_id = test_character_id();
        let mut original = test_character(character_id, ruleset_id, TEST_SCHEMA)?;

        repository.create(&original)?;
        let result = original.take_damage(3);
        assert_eq!(result.temporary_lost, 3);
        let stored = repository.load(character_id)?;
        assert_ne!(stored.state().hit_points(), original.state().hit_points());

        Ok(())
    }

    #[test]
    fn update_replaces_character_snapshot() -> TestResult {
        let (mut repository, ruleset_id) = repository_witn_ruleset()?;
        let character_id = test_character_id();
        let mut original = test_character(character_id, ruleset_id, TEST_SCHEMA)?;

        repository.create(&original)?;
        let result = original.take_damage(3);
        assert_eq!(result.temporary_lost, 3);

        repository.update(&original)?;

        let stored = repository.load(character_id)?;
        assert_eq!(original.state().hit_points(), stored.state().hit_points());

        Ok(())
    }

    #[test]
    fn updating_missing_character_is_rejected() -> TestResult {
        let (mut repository, ruleset_id) = repository_witn_ruleset()?;
        let character_id = test_character_id();
        let character = test_character(character_id, ruleset_id.clone(), TEST_SCHEMA)?;
        let unknown_id = CharacterId::new();
        let unknown = test_character(unknown_id, ruleset_id, TEST_SCHEMA)?;

        repository.create(&character)?;

        let result = repository.update(&unknown);
        assert!(matches!(
            result,
            Err(PersistenceError::CharacterNotFound { id }) if id == unknown_id
        ));

        Ok(())
    }

    #[test]
    fn lists_sorted_after_out_of_order_store() -> TestResult {
        let (mut repository, ruleset_id) = repository_witn_ruleset()?;
        let id1 = CharacterId::from_uuid(Uuid::from_u128(1_u128));
        let id2 = CharacterId::from_uuid(Uuid::from_u128(2_u128));
        let id3 = CharacterId::from_uuid(Uuid::from_u128(3_u128));
        let ids = [id1, id2, id3];

        repository.create(&test_character(id3, ruleset_id.clone(), TEST_SCHEMA)?)?;
        repository.create(&test_character(id1, ruleset_id.clone(), TEST_SCHEMA)?)?;
        repository.create(&test_character(id2, ruleset_id, TEST_SCHEMA)?)?;

        let list = repository.list()?;

        for (idx, id) in list.iter().enumerate() {
            assert_eq!(id, ids.get(idx).expect("should be present"));
        }

        Ok(())
    }

    #[test]
    fn delete_only_touches_specified_record() -> TestResult {
        let (mut repository, ruleset_id) = repository_witn_ruleset()?;
        let id1 = CharacterId::from_uuid(Uuid::from_u128(1_u128));
        let id2 = CharacterId::from_uuid(Uuid::from_u128(2_u128));

        repository.create(&test_character(id1, ruleset_id.clone(), TEST_SCHEMA)?)?;
        repository.create(&test_character(id2, ruleset_id, TEST_SCHEMA)?)?;

        repository.delete(id1)?;

        let list = repository.list()?;
        assert_eq!(list.len(), 1);

        repository.load(id2)?;

        Ok(())
    }

    #[test]
    fn deleting_missing_id_keeps_repository_records() -> TestResult {
        let (mut repository, ruleset_id) = repository_witn_ruleset()?;
        let id1 = CharacterId::from_uuid(Uuid::from_u128(1_u128));
        let id2 = CharacterId::from_uuid(Uuid::from_u128(2_u128));
        let missing = CharacterId::new();

        repository.create(&test_character(id1, ruleset_id.clone(), TEST_SCHEMA)?)?;
        repository.create(&test_character(id2, ruleset_id, TEST_SCHEMA)?)?;

        let result = repository.delete(missing);
        assert!(matches!(result, Err(PersistenceError::CharacterNotFound { id }) if id == missing));

        let list = repository.list()?;
        assert_eq!(list.len(), 2);

        repository.load(id1)?;
        repository.load(id2)?;

        Ok(())
    }
}
