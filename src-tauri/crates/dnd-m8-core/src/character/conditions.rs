use core::fmt;
use std::collections::{btree_map::Entry, BTreeMap};

use uuid::Uuid;

use crate::{CoreError, CoreResult};

/// Stable identifier of one ruleset-defined condition.
///
/// The owning ruleset interprets identifiers such as `condition.poisoned`.
/// Core only stores and compares them.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConditionId(String);

impl ConditionId {
    /// Constructs a normalized condition identifier.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::EmptyConditionId`] when the normalized identifier
    /// is empty.
    pub fn new(value: impl Into<String>) -> CoreResult<Self> {
        let value = value.into();
        let value = value.trim();

        if value.is_empty() {
            return Err(Box::new(CoreError::EmptyConditionId));
        }

        Ok(Self(value.to_owned()))
    }

    /// Returns the textual identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ConditionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Unique identifier of one runtime condition application.
///
/// Multiple applications may reference the same [`ConditionId`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConditionApplicationId(Uuid);

impl ConditionApplicationId {
    /// Generates a new runtime application identifier.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Constructs an application identifier from an existing UUID.
    #[must_use]
    pub const fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    /// Returns the underlying UUID.
    #[must_use]
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ConditionApplicationId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ConditionApplicationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// One active application of a ruleset-defined condition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveCondition {
    /// Unique runtime application identifier.
    application_id: ConditionApplicationId,
    /// Ruleset-defined condition identifier.
    condition_id: ConditionId,
}

impl ActiveCondition {
    /// Creates a new active condition application.
    #[must_use]
    pub fn new(condition_id: ConditionId) -> Self {
        Self {
            application_id: ConditionApplicationId::new(),
            condition_id,
        }
    }

    /// Reconstructs an existing condition application.
    ///
    /// This is primarily intended for deterministic tests and future
    /// persistence loading.
    #[must_use]
    pub const fn from_parts(
        application_id: ConditionApplicationId,
        condition_id: ConditionId,
    ) -> Self {
        Self {
            application_id,
            condition_id,
        }
    }

    /// Returns the unique runtime application identifier.
    #[must_use]
    pub const fn application_id(&self) -> ConditionApplicationId {
        self.application_id
    }

    /// Returns the ruleset-defined condition identifier.
    #[must_use]
    pub const fn condition_id(&self) -> &ConditionId {
        &self.condition_id
    }
}

/// Result of applying a condition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionApplyOutcome {
    /// Identifier of the newly stored application.
    pub application_id: ConditionApplicationId,
    /// Ruleset-defined condition identifier.
    pub condition_id: ConditionId,
    /// Whether this was the first active application of the condition.
    pub became_active: bool,
    /// Number of active applications after the operation.
    pub active_applications: usize,
}

/// Result of removing a condition application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionRemoveOutcome {
    /// Identifier of the removed application.
    pub application_id: ConditionApplicationId,
    /// Ruleset-defined condition identifier.
    pub condition_id: ConditionId,
    /// Whether another application keeps the condition active.
    pub remains_active: bool,
    /// Number of remaining applications of the same condition.
    pub remaining_applications: usize,
}

/// Collection of active condition applications.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ActiveConditions {
    /// Applications indexed by their unique runtime identifiers.
    applications: BTreeMap<ConditionApplicationId, ActiveCondition>,
}

impl ActiveConditions {
    /// Constructs an empty active-condition collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Applies a ruleset-defined condition.
    ///
    /// # Errors
    ///
    /// Returns an error in the extremely unlikely event that the generated
    /// application identifier is already present.
    pub fn apply(&mut self, condition_id: ConditionId) -> CoreResult<ConditionApplyOutcome> {
        self.insert(ActiveCondition::new(condition_id))
    }

    /// Inserts an existing condition application.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::DuplicateConditionApplication`] when the exact
    /// runtime application identifier is already present.
    pub fn insert(&mut self, condition: ActiveCondition) -> CoreResult<ConditionApplyOutcome> {
        let application_id = condition.application_id();
        let condition_id = condition.condition_id().clone();
        let became_active = !self.is_active(&condition_id);

        match self.applications.entry(application_id) {
            Entry::Occupied(_) => {
                return Err(Box::new(CoreError::DuplicateConditionApplication {
                    id: application_id,
                }));
            }
            Entry::Vacant(entry) => {
                entry.insert(condition);
            }
        }

        Ok(ConditionApplyOutcome {
            application_id,
            active_applications: self.application_count(&condition_id),
            condition_id,
            became_active,
        })
    }

    /// Removes one condition application.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::ConditionApplicationNotFound`] when the requested
    /// application is unavailable.
    pub fn remove(
        &mut self,
        application_id: ConditionApplicationId,
    ) -> CoreResult<ConditionRemoveOutcome> {
        let removed = self.applications.remove(&application_id).ok_or_else(|| {
            Box::new(CoreError::ConditionApplicationNotFound { id: application_id })
        })?;

        let condition_id = removed.condition_id;
        let remaining_applications = self.application_count(&condition_id);

        Ok(ConditionRemoveOutcome {
            application_id,
            condition_id,
            remains_active: remaining_applications > 0,
            remaining_applications,
        })
    }

    /// Returns an application by its runtime identifier.
    #[must_use]
    pub fn condition(&self, application_id: ConditionApplicationId) -> Option<&ActiveCondition> {
        self.applications.get(&application_id)
    }

    /// Returns whether at least one application of a condition is active.
    #[must_use]
    pub fn is_active(&self, condition_id: &ConditionId) -> bool {
        self.applications
            .values()
            .any(|condition| condition.condition_id() == condition_id)
    }

    /// Returns the number of active applications of a condition.
    #[must_use]
    pub fn application_count(&self, condition_id: &ConditionId) -> usize {
        self.applications
            .values()
            .filter(|condition| condition.condition_id() == condition_id)
            .count()
    }

    /// Iterates over active applications in stable identifier order.
    #[must_use]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &ActiveCondition> {
        self.applications.values()
    }

    /// Returns the total number of active applications.
    #[must_use]
    pub fn len(&self) -> usize {
        self.applications.len()
    }

    /// Returns whether no condition applications are active.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.applications.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::{ActiveCondition, ActiveConditions, ConditionId, CoreResult};
    use pretty_assertions::assert_eq;

    #[test]
    fn condition_id_trimms_whitespaces() -> CoreResult<()> {
        let id = ConditionId::new(" arrow in the knee ")?;

        assert_eq!(id.as_str(), "arrow in the knee");

        Ok(())
    }

    #[test]
    fn empty_ids_are_rejected() {
        ConditionId::new("").expect_err("empty id should be rejected");
    }

    #[test]
    fn duplicate_ids_are_rejected() -> CoreResult<()> {
        let mut conditions = ActiveConditions::new();
        let condition = ActiveCondition::new(ConditionId::new("condition.1")?);

        conditions.insert(condition.clone())?;
        conditions
            .insert(condition)
            .expect_err("duplicate application id should be rejected");

        Ok(())
    }

    #[test]
    fn missing_condition_removal_fails() -> CoreResult<()> {
        let mut conditions = ActiveConditions::new();
        let condition = ActiveCondition::new(ConditionId::new("condition.1")?);
        let application_id = condition.application_id();

        conditions
            .remove(application_id)
            .expect_err("removing missing applied condition should fail");

        conditions.insert(condition)?;

        conditions
            .remove(application_id)
            .expect("removing a present applied condition should pass");

        Ok(())
    }

    #[test]
    fn removing_one_application_keeps_condition_active() -> CoreResult<()> {
        let condition_id = ConditionId::new("condition.poisoned")?;
        let mut conditions = ActiveConditions::new();

        let first = conditions.apply(condition_id.clone())?;
        let second = conditions.apply(condition_id.clone())?;

        assert!(first.became_active);
        assert!(!second.became_active);
        assert_eq!(conditions.application_count(&condition_id), 2);

        let removed = conditions.remove(first.application_id)?;

        assert!(removed.remains_active);
        assert_eq!(removed.remaining_applications, 1);
        assert!(conditions.is_active(&condition_id));

        Ok(())
    }

    #[test]
    fn distinct_conditions_should_not_affect_eachother() -> CoreResult<()> {
        let mut conditions = ActiveConditions::new();
        let id1 = ConditionId::new("condition.1")?;
        let id2 = ConditionId::new("condition.2")?;

        let mut result = conditions.apply(id1.clone())?;
        assert_eq!(result.condition_id, id1);
        assert_eq!(conditions.application_count(&id1), 1);
        assert_eq!(conditions.application_count(&id2), 0);

        result = conditions.apply(id2.clone())?;
        assert_eq!(result.condition_id, id2);

        assert_eq!(conditions.application_count(&id1), 1);
        assert_eq!(conditions.application_count(&id2), 1);

        let _ = conditions.apply(id1.clone())?;
        result = conditions.apply(id2.clone())?;

        assert_eq!(conditions.application_count(&id1), 2);
        assert_eq!(conditions.application_count(&id2), 2);

        let remove_result = conditions.remove(result.application_id)?;

        assert_eq!(remove_result.condition_id, id2);

        assert_eq!(conditions.application_count(&id1), 2);
        assert_eq!(conditions.application_count(&id2), 1);

        Ok(())
    }

    #[test]
    fn iteration_includes_every_application() -> CoreResult<()> {
        let mut conditions = ActiveConditions::new();
        let id1 = ConditionId::new("condition.1")?;
        let id2 = ConditionId::new("condition.2")?;
        let ids = [
            conditions.apply(id1)?.application_id,
            conditions.apply(id2.clone())?.application_id,
            conditions.apply(id2)?.application_id,
        ];
        let mut iterated = Vec::new();

        assert_eq!(conditions.len(), 3);

        for condition in conditions.iter() {
            let id = &condition.application_id;
            assert!(ids.contains(id));
            assert!(!iterated.contains(id));
            iterated.push(*id);
        }

        Ok(())
    }
}
