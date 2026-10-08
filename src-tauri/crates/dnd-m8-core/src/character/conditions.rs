use super::{Instance, Instances};
use crate::{CoreError, CoreResult, DefinitionId, InstanceId};

/// Compile-time domain marker for condition identifiers.
///
/// This type has no runtime values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConditionKind {}

/// Stable identifier of one ruleset-defined condition.
///
/// Core only stores and compares this identity, the owning ruleset
/// interprets it.
pub type ConditionId = DefinitionId<ConditionKind>;

/// Unique identifier of one runtime condition instance.
///
/// Multiple instances may reference the same [`ConditionId`].
pub type ConditionInstanceId = InstanceId<ConditionKind>;

impl DefinitionId<ConditionKind> {
    /// Constructs a new `ConditionId` from a text identifier.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::EmptyConditionId`] on empty string value.
    pub fn new(value: impl Into<String>) -> CoreResult<Self> {
        Self::from_text(value, CoreError::EmptyConditionId)
    }
}

/// One active instance of a ruleset-defined condition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveCondition {
    /// Unique runtime instance identifier.
    instance_id: ConditionInstanceId,
    /// Ruleset-defined condition identifier.
    condition_id: ConditionId,
}

impl ActiveCondition {
    /// Creates a new active condition instance.
    #[must_use]
    pub fn new(condition_id: ConditionId) -> Self {
        Self {
            instance_id: ConditionInstanceId::new(),
            condition_id,
        }
    }

    /// Reconstructs an existing condition instance.
    ///
    /// This is primarily intended for deterministic tests and future
    /// persistence loading.
    #[must_use]
    pub const fn from_parts(instance_id: ConditionInstanceId, condition_id: ConditionId) -> Self {
        Self {
            instance_id,
            condition_id,
        }
    }
}

impl Instance for ActiveCondition {
    type InstanceId = ConditionInstanceId;
    type DefinitionId = ConditionId;

    fn instance_id(&self) -> Self::InstanceId {
        self.instance_id
    }

    fn definition_id(&self) -> &Self::DefinitionId {
        &self.condition_id
    }
}

/// Result of applying a condition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionApplyOutcome {
    /// Identifier of the newly stored instance.
    pub instance_id: ConditionInstanceId,
    /// Ruleset-defined condition identifier.
    pub condition_id: ConditionId,
    /// Whether this was the first active instance of the condition.
    pub became_active: bool,
    /// Number of active instances after the operation.
    pub active_instances: usize,
}

/// Result of removing a condition instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionRemoveOutcome {
    /// Identifier of the removed instance.
    pub instance_id: ConditionInstanceId,
    /// Ruleset-defined condition identifier.
    pub condition_id: ConditionId,
    /// Whether another instance keeps the condition active.
    pub remains_active: bool,
    /// Number of remaining instances of the same condition.
    pub remaining_instances: usize,
}

/// Collection of active condition instances.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ActiveConditions {
    /// Instances indexed by their unique runtime identifiers.
    instances: Instances<ActiveCondition>,
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
    /// instance identifier is already present.
    pub fn apply(&mut self, condition_id: ConditionId) -> CoreResult<ConditionApplyOutcome> {
        self.insert(ActiveCondition::new(condition_id))
    }

    /// Inserts an existing condition instance.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::DuplicateConditionInstance`] when the exact
    /// runtime instance identifier is already present.
    pub fn insert(&mut self, condition: ActiveCondition) -> CoreResult<ConditionApplyOutcome> {
        let instance_id = condition.instance_id();
        let condition_id = condition.condition_id.clone();

        self.instances.insert(condition).map_err(|duplicate| {
            Box::new(CoreError::DuplicateConditionInstance { id: duplicate.id })
        })?;

        let active_instances = self.instances.instance_count(&condition_id);

        Ok(ConditionApplyOutcome {
            instance_id,
            active_instances,
            condition_id,
            became_active: active_instances == 1,
        })
    }

    /// Removes one condition instance.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::ConditionInstanceNotFound`] when the requested
    /// instance is unavailable.
    pub fn remove(
        &mut self,
        instance_id: ConditionInstanceId,
    ) -> CoreResult<ConditionRemoveOutcome> {
        let removed = self
            .instances
            .remove(instance_id)
            .ok_or_else(|| Box::new(CoreError::ConditionInstanceNotFound { id: instance_id }))?;

        let condition_id = removed.condition_id;
        let remaining_instances = self.instance_count(&condition_id);

        Ok(ConditionRemoveOutcome {
            instance_id,
            condition_id,
            remains_active: remaining_instances > 0,
            remaining_instances,
        })
    }

    /// Returns an instance by its runtime identifier.
    #[must_use]
    pub fn condition(&self, instance_id: ConditionInstanceId) -> Option<&ActiveCondition> {
        self.instances.get(instance_id)
    }

    /// Returns whether at least one instance of a condition is active.
    #[must_use]
    pub fn is_active(&self, condition_id: &ConditionId) -> bool {
        self.instances.is_active(condition_id)
    }

    /// Returns the number of active instances of a condition.
    #[must_use]
    pub fn instance_count(&self, condition_id: &ConditionId) -> usize {
        self.instances.instance_count(condition_id)
    }

    /// Iterates over active instances in stable identifier order.
    #[must_use]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &ActiveCondition> {
        self.instances.iter()
    }

    /// Returns the total number of active instances.
    #[must_use]
    pub fn len(&self) -> usize {
        self.instances.len()
    }

    /// Returns whether no condition instances are active.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        character::runtime_instance::Instance, ActiveCondition, ActiveConditions, ConditionId,
        CoreResult,
    };
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
            .expect_err("duplicate instance id should be rejected");

        Ok(())
    }

    #[test]
    fn missing_condition_removal_fails() -> CoreResult<()> {
        let mut conditions = ActiveConditions::new();
        let condition = ActiveCondition::new(ConditionId::new("condition.1")?);
        let instance_id = condition.instance_id();

        conditions
            .remove(instance_id)
            .expect_err("removing missing applied condition should fail");

        conditions.insert(condition)?;

        conditions
            .remove(instance_id)
            .expect("removing a present applied condition should pass");

        Ok(())
    }

    #[test]
    fn removing_one_instance_keeps_condition_active() -> CoreResult<()> {
        let condition_id = ConditionId::new("condition.poisoned")?;
        let mut conditions = ActiveConditions::new();

        let first = conditions.apply(condition_id.clone())?;
        let second = conditions.apply(condition_id.clone())?;

        assert!(first.became_active);
        assert!(!second.became_active);
        assert_eq!(conditions.instance_count(&condition_id), 2);

        let removed = conditions.remove(first.instance_id)?;

        assert!(removed.remains_active);
        assert_eq!(removed.remaining_instances, 1);
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
        assert_eq!(conditions.instance_count(&id1), 1);
        assert_eq!(conditions.instance_count(&id2), 0);

        result = conditions.apply(id2.clone())?;
        assert_eq!(result.condition_id, id2);

        assert_eq!(conditions.instance_count(&id1), 1);
        assert_eq!(conditions.instance_count(&id2), 1);

        let _ = conditions.apply(id1.clone())?;
        result = conditions.apply(id2.clone())?;

        assert_eq!(conditions.instance_count(&id1), 2);
        assert_eq!(conditions.instance_count(&id2), 2);

        let remove_result = conditions.remove(result.instance_id)?;

        assert_eq!(remove_result.condition_id, id2);

        assert_eq!(conditions.instance_count(&id1), 2);
        assert_eq!(conditions.instance_count(&id2), 1);

        Ok(())
    }

    #[test]
    fn iteration_includes_every_instance() -> CoreResult<()> {
        let mut conditions = ActiveConditions::new();
        let id1 = ConditionId::new("condition.1")?;
        let id2 = ConditionId::new("condition.2")?;
        let ids = [
            conditions.apply(id1)?.instance_id,
            conditions.apply(id2.clone())?.instance_id,
            conditions.apply(id2)?.instance_id,
        ];
        let mut iterated = Vec::new();

        assert_eq!(conditions.len(), 3);

        for condition in conditions.iter() {
            let id = &condition.instance_id;
            assert!(ids.contains(id));
            assert!(!iterated.contains(id));
            iterated.push(*id);
        }

        Ok(())
    }
}
