use core::fmt;
use std::collections::{btree_map::Entry, BTreeMap};

use crate::{CoreError, CoreResult};

/// Stable identifier of one ruleset-defined character resource.
///
/// Identifiers are meaningfull only to the owning ruleset. Core stores and
/// compares without interpreting their contents.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceId(String);

impl ResourceId {
    /// Constructs a normalized resource identifier.
    ///
    /// Leading and trailing whitespace is removed.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::EmptyResourceId`] when the resulting identifier is
    /// empty.
    pub fn new(value: impl Into<String>) -> CoreResult<Self> {
        let value = value.into();
        let value = value.trim();

        if value.is_empty() {
            return Err(Box::new(CoreError::EmptyResourceId));
        }

        Ok(Self(value.to_owned()))
    }

    /// Returns the resource identifier as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Current stat of one limited character resource.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourcePool {
    /// Stable ruleset-defined identifier.
    id: ResourceId,
    /// Currently available ammout.
    current: u32,
    /// Current maximum ammount.
    maximum: u32,
}

impl ResourcePool {
    /// Constructs a validated resource pool.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::InvalidResourcePool`] when `current` exceeds
    /// `maximum`.
    pub fn new(id: ResourceId, current: u32, maximum: u32) -> CoreResult<Self> {
        if current > maximum {
            return Err(Box::new(CoreError::InvalidResourcePool {
                id,
                current,
                maximum,
            }));
        }

        Ok(Self {
            id,
            current,
            maximum,
        })
    }

    /// Returns the stable source identifier.
    #[must_use]
    pub const fn id(&self) -> &ResourceId {
        &self.id
    }

    /// Returns the current available ammount.
    #[must_use]
    pub const fn current(&self) -> u32 {
        self.current
    }

    /// Returns the current maximum ammount.
    #[must_use]
    pub const fn maximum(&self) -> u32 {
        self.maximum
    }

    /// Spends an amount of this resource.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::InsufficientResource`] without changing the pool
    /// when the requested amount exceeds the amount available.
    pub fn spend(&mut self, amount: u32) -> CoreResult<ResourceSpendOutcome> {
        if amount > self.current {
            return Err(Box::new(CoreError::InsufficientResource {
                id: self.id.clone(),
                requested: amount,
                available: self.current,
            }));
        }

        let previous = self.current;
        self.current = self.current.saturating_sub(amount);

        Ok(ResourceSpendOutcome {
            previous,
            spent: amount,
            current: self.current,
        })
    }

    /// Restores an amount of this resource, capped at its maximum.
    #[must_use]
    pub fn restore(&mut self, amount: u32) -> ResourceRestoreOutcome {
        let previous = self.current;
        let available = self.maximum.saturating_sub(self.current);
        let restored = amount.min(available);

        self.current = self.current.saturating_add(restored);

        ResourceRestoreOutcome {
            requested: amount,
            previous,
            restored,
            current: self.current,
            excess: amount.saturating_sub(restored),
        }
    }
}

/// Result of successfully spending a resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceSpendOutcome {
    /// Available ammount.
    pub previous: u32,
    /// Spent ammount.
    pub spent: u32,
    /// Current ammount after expenditiure.
    pub current: u32,
}

/// Result of restoring a resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceRestoreOutcome {
    /// Amount requested by the caller.
    pub requested: u32,
    /// Available amount before restoration.
    pub previous: u32,
    /// Amount actually restored.
    pub restored: u32,
    /// Available amount after restoration.
    pub current: u32,
    /// Unused restoration after reaching the maximum.
    pub excess: u32,
}

/// Ruleset-independent collection of character resource pools.
///
/// A `BTreeMap` provides deterministic ordering for future persistence and
/// frontend projections.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ResourcePools {
    /// Pools indexed by their stable ruleset-provided identifiers.
    pools: BTreeMap<ResourceId, ResourcePool>,
}

impl ResourcePools {
    /// Constructs an empty resource collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a resource pool.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::DuplicateResource`] when the identifier is already
    /// present.
    pub fn insert(&mut self, pool: ResourcePool) -> CoreResult<()> {
        let id = pool.id().clone();

        match self.pools.entry(id.clone()) {
            Entry::Occupied(_) => Err(Box::new(CoreError::DuplicateResource { id })),
            Entry::Vacant(entry) => {
                entry.insert(pool);
                Ok(())
            }
        }
    }

    /// Returns a resource by identifier.
    #[must_use]
    pub fn resource(&self, id: &ResourceId) -> Option<&ResourcePool> {
        self.pools.get(id)
    }

    /// Iterates over resources in stable identifier order.
    #[must_use]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &ResourcePool> {
        self.pools.values()
    }

    /// Returns the number of resource pools.
    #[must_use]
    pub fn len(&self) -> usize {
        self.pools.len()
    }

    /// Returns whether no resource pools are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pools.is_empty()
    }

    /// Spends a resource.
    ///
    /// # Errors
    ///
    /// Returns an error when the resource is missing or insufficient.
    pub fn spend(&mut self, id: &ResourceId, amount: u32) -> CoreResult<ResourceSpendOutcome> {
        self.resource_mut(id)?.spend(amount)
    }

    /// Restores a resource.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::ResourceNotFound`] when the resource is missing.
    pub fn restore(&mut self, id: &ResourceId, amount: u32) -> CoreResult<ResourceRestoreOutcome> {
        Ok(self.resource_mut(id)?.restore(amount))
    }

    /// Returns a mutable resource or a contextual missing-resource error.
    fn resource_mut(&mut self, id: &ResourceId) -> CoreResult<&mut ResourcePool> {
        self.pools
            .get_mut(id)
            .ok_or_else(|| Box::new(CoreError::ResourceNotFound { id: id.clone() }))
    }
}

#[cfg(test)]
mod tests {
    use crate::{CoreResult, ResourceId, ResourcePool, ResourcePools};
    use pretty_assertions::assert_eq;

    #[test]
    fn id_trims_surrounding_whitespace() -> CoreResult<()> {
        let id = ResourceId::new(" kung foo  ")?;

        assert_eq!(id.as_str(), "kung foo");

        Ok(())
    }

    #[test]
    fn whitespace_only_id_rejected() {
        ResourceId::new("  ").expect_err("whitespace only id should be rejected");
    }

    #[test]
    fn zero_current_and_zero_max_pools_valid() -> CoreResult<()> {
        let resource = ResourcePool::new(ResourceId::new("zero max")?, 0, 0)?;

        assert_eq!(resource.current(), 0);
        assert_eq!(resource.maximum(), 0);

        Ok(())
    }

    #[test]
    fn current_above_maximum_is_rejected() {
        ResourcePool::new(
            ResourceId::new("current above max").expect("id should be fine"),
            5,
            1,
        )
        .expect_err("current above max should be rejected");
    }

    #[test]
    fn spendig_resource_reports_correct_values() -> CoreResult<()> {
        let mut resource = ResourcePool::new(ResourceId::new("ki points")?, 5, 5)?;

        let mut outcome = resource.spend(2)?;

        assert_eq!(outcome.previous, 5);
        assert_eq!(outcome.spent, 2);
        assert_eq!(outcome.current, 3);

        outcome = resource.spend(3)?;

        assert_eq!(outcome.spent, 3);
        assert_eq!(outcome.current, 0);

        Ok(())
    }

    #[test]
    fn overspending_fails() -> CoreResult<()> {
        let mut resource = ResourcePool::new(ResourceId::new("ki points")?, 2, 5)?;

        resource.spend(5).expect_err("overshpending should fail");

        Ok(())
    }

    #[test]
    fn restoration_caps_at_max() -> CoreResult<()> {
        let mut resource = ResourcePool::new(ResourceId::new("ki points")?, 2, 5)?;

        let outcome = resource.restore(6);

        assert_eq!(outcome.previous, 2);
        assert_eq!(outcome.current, 5);
        assert_eq!(outcome.excess, 3);

        Ok(())
    }

    #[test]
    fn duplicate_ids_are_rejected() -> CoreResult<()> {
        let mut pool = ResourcePools::new();

        pool.insert(ResourcePool::new(ResourceId::new("ki points")?, 5, 5)?)?;
        pool.insert(ResourcePool::new(ResourceId::new("ki points")?, 5, 5)?)
            .expect_err("duplicate resource id should be rejected");

        Ok(())
    }

    #[test]
    fn missing_id_lookup_fails() -> CoreResult<()> {
        let mut pool = ResourcePools::new();

        pool.spend(&ResourceId::new("nonexistent")?, 9)
            .expect_err("Missing resource id lookup should fail");

        pool.restore(&ResourceId::new("nonexistent")?, 9)
            .expect_err("Missing resource id lookup should fail");

        pool.resource_mut(&ResourceId::new("nonexistent")?)
            .expect_err("Missing resource id lookup should fail");

        Ok(())
    }

    #[test]
    fn pool_iteration_is_deterministic() -> CoreResult<()> {
        let mut pool = ResourcePools::new();
        pool.insert(ResourcePool::new(ResourceId::new("0")?, 0, 0)?)?;
        pool.insert(ResourcePool::new(ResourceId::new("1")?, 0, 0)?)?;
        pool.insert(ResourcePool::new(ResourceId::new("2")?, 0, 0)?)?;

        for (idx, resource) in pool.iter().enumerate() {
            assert_eq!(resource.id(), &ResourceId::new(idx.to_string())?);
        }

        for (idx, resource) in pool.iter().enumerate() {
            assert_eq!(resource.id(), &ResourceId::new(idx.to_string())?);
        }

        Ok(())
    }
}
