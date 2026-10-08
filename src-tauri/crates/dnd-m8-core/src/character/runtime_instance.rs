use std::{
    collections::{btree_map::Entry, BTreeMap},
    fmt::Debug,
};

/// Identity required by a shared runtime-instance collection.
pub trait Instance {
    /// Unique identifier of one runtime instance.
    type InstanceId: Copy + Ord + Debug;

    /// Ruleset-defined identity shared by related instances.
    type DefinitionId: Eq;

    /// Returns the instance's unique identifier.
    fn instance_id(&self) -> Self::InstanceId;

    /// Returns the instance's ruleset-defined identity.
    fn definition_id(&self) -> &Self::DefinitionId;
}

/// Rejected identifier of an instance already present in the collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DuplicateInstance<Id> {
    /// Runtime identifier already in use.
    pub id: Id,
}

/// Instance indexed by their unique runtime identifier.
///
/// Multiple instances may reference the same ruleset definition. Iteration
/// follows runtime identifier order, not instance order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instances<I: Instance> {
    /// Stored instances indexed by runtime ids.
    entries: BTreeMap<I::InstanceId, I>,
}

impl<I: Instance> Instances<I> {
    /// Constructs a new storage pool of runtime instances.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    /// Inserts an instance without replacing an existing entry.
    ///
    /// # Errors
    ///
    /// Returns the duplicate runtime identifier when an instance already uses it.
    pub fn insert(&mut self, instance: I) -> Result<(), DuplicateInstance<I::InstanceId>> {
        let id = instance.instance_id();

        match self.entries.entry(id) {
            Entry::Occupied(_) => Err(DuplicateInstance { id }),
            Entry::Vacant(entry) => {
                entry.insert(instance);
                Ok(())
            }
        }
    }

    /// Removes and returns an instance, if present.
    pub fn remove(&mut self, id: I::InstanceId) -> Option<I> {
        self.entries.remove(&id)
    }

    /// Returns an instance by its runtime identifier.
    pub fn get(&self, id: I::InstanceId) -> Option<&I> {
        self.entries.get(&id)
    }

    /// Returns whether any instances reference the definition.
    pub fn is_active(&self, definition_id: &I::DefinitionId) -> bool {
        self.entries
            .values()
            .any(|instance| instance.definition_id() == definition_id)
    }

    /// Returns the number of instances referencing the definition.
    pub fn instance_count(&self, definition_id: &I::DefinitionId) -> usize {
        self.entries
            .values()
            .filter(|instance| instance.definition_id() == definition_id)
            .count()
    }

    /// Iterates over instances in runtime id order.
    #[must_use]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &I> {
        self.entries.values()
    }

    /// Returns the total number of instances.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether no instances are stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl<I: Instance> Default for Instances<I> {
    fn default() -> Self {
        Self::new()
    }
}
