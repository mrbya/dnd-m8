use crate::{CoreError, CoreResult, DefinitionId, EffectDuration, Instance, InstanceId, Instances};

/// Compile-time domain marker for effect identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EffectKind {}

/// Stable identifier of one ruleset-defined effect.
pub type EffectId = DefinitionId<EffectKind>;

/// Unique identifier of one runtime effect instance.
pub type EffectInstanceId = InstanceId<EffectKind>;

impl DefinitionId<EffectKind> {
    /// Constructs a normalized effect identifier.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::EmptyEffectId`] when the trimmed provided value is empty.
    pub fn new(value: impl Into<String>) -> CoreResult<Self> {
        Self::from_text(value, CoreError::EmptyEffectId)
    }
}

/// One active instance of a ruleset-defined effect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveEffect {
    /// Unique runtime instance identifier.
    instance_id: EffectInstanceId,
    /// Ruleset-defined effect identifier.
    effect_id: EffectId,
    /// Remaning lifetime of this instance.
    duration: EffectDuration,
}

impl ActiveEffect {
    /// Creates a new active effect instance.
    #[must_use]
    pub fn new(effect_id: EffectId, duration: EffectDuration) -> Self {
        Self::from_parts(EffectInstanceId::new(), effect_id, duration)
    }

    /// Reconstructs effect instance from parts.
    #[must_use]
    pub const fn from_parts(
        instance_id: EffectInstanceId,
        effect_id: EffectId,
        duration: EffectDuration,
    ) -> Self {
        Self {
            instance_id,
            effect_id,
            duration,
        }
    }

    /// Returns the ruleset-defined effect identifier.
    #[must_use]
    pub const fn effect_id(&self) -> &EffectId {
        &self.effect_id
    }

    /// Returns the remaining lifetime.
    #[must_use]
    pub const fn duration(&self) -> EffectDuration {
        self.duration
    }
}

impl Instance for ActiveEffect {
    type InstanceId = EffectInstanceId;
    type DefinitionId = EffectId;

    fn instance_id(&self) -> Self::InstanceId {
        self.instance_id
    }

    fn definition_id(&self) -> &Self::DefinitionId {
        &self.effect_id
    }
}

/// Result of applying an effect instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectApplyOutcome {
    /// Identifier of the stored instance.
    pub instance_id: EffectInstanceId,
    /// Ruleset-defined effect identifier.
    pub effect_id: EffectId,
    /// Lifetime assigned to the instance.
    pub duration: EffectDuration,
    /// Whether this is the first active instance of the effect.
    pub became_active: bool,
    /// Number of active instances of the same effect.
    pub active_instances: usize,
}

/// Result of removing an effect instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectRemoveOutcome {
    /// Identifier of the removed instance.
    pub instance_id: EffectInstanceId,
    /// Ruleset-defined effect identifier.
    pub effect_id: EffectId,
    /// Duration immediately before removal.
    pub duration: EffectDuration,
    /// Whether another instance keeps the effect active.
    pub remains_active: bool,
    /// Number of remaining instances of the same effect.
    pub remaining_instances: usize,
}

/// Result of advancing one effect instance's countdown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectAdvanceOutcome {
    /// Identifier of the advanced instance.
    pub instance_id: EffectInstanceId,
    /// Ruleset-defined effect identifier.
    pub effect_id: EffectId,
    /// Rounds consumed from the finite countdown.
    pub elapsed_rounds: u32,
    /// Remaining duration, or none when the instance expired.
    pub remaining_duration: Option<EffectDuration>,
}

/// Collection of independently tracked active effect instances.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ActiveEffects {
    /// Shared storage indexed by runtime identity.
    instances: Instances<ActiveEffect>,
}

impl ActiveEffects {
    /// Constructs an empty active-effect collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Applies a new effect instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the generated instance identifier already exists.
    pub fn apply(
        &mut self,
        effect_id: EffectId,
        duration: EffectDuration,
    ) -> CoreResult<EffectApplyOutcome> {
        self.insert(ActiveEffect::new(effect_id, duration))
    }

    /// Inserts an existing effect instance without replacing another.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::DuplicateEffectInstance`] for a duplicate identity.
    pub fn insert(&mut self, effect: ActiveEffect) -> CoreResult<EffectApplyOutcome> {
        let instance_id = effect.instance_id;
        let effect_id = effect.effect_id.clone();
        let duration = effect.duration;

        self.instances.insert(effect).map_err(|duplicate| {
            Box::new(CoreError::DuplicateEffectInstance { id: duplicate.id })
        })?;

        let active_instances = self.instances.instance_count(&effect_id);

        Ok(EffectApplyOutcome {
            instance_id,
            effect_id,
            duration,
            became_active: active_instances == 1,
            active_instances,
        })
    }

    /// Removes exactly one effect instance.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::EffectInstanceNotFound`] when the instance is absent.
    pub fn remove(&mut self, instance_id: EffectInstanceId) -> CoreResult<EffectRemoveOutcome> {
        let removed = self
            .instances
            .remove(instance_id)
            .ok_or_else(|| Box::new(CoreError::EffectInstanceNotFound { id: instance_id }))?;

        let remaining_instances = self.instances.instance_count(&removed.effect_id);

        Ok(EffectRemoveOutcome {
            instance_id,
            effect_id: removed.effect_id,
            duration: removed.duration,
            remains_active: remaining_instances > 0,
            remaining_instances,
        })
    }

    /// Advances exactly one effect instance's round countdown.
    ///
    /// Expiration removes the instance. Advancing by zero is a no-op.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::EffectInstanceNotFound`] when the instance is absent.
    pub fn advance_rounds(
        &mut self,
        instance_id: EffectInstanceId,
        rounds: u32,
    ) -> CoreResult<EffectAdvanceOutcome> {
        let effect = self
            .instances
            .get(instance_id)
            .ok_or_else(|| Box::new(CoreError::EffectInstanceNotFound { id: instance_id }))?;

        let effect_id = effect.effect_id.clone();
        let advance = effect.duration.advance_rounds(rounds);

        if advance.remaining_duration != Some(effect.duration) {
            // Identity stays unchanged; only the duration is updated.
            //
            // Use a dedicated shared-storage mutation operation here,
            // as described below.
            self.update_duration(instance_id, advance.remaining_duration)?;
        }

        Ok(EffectAdvanceOutcome {
            instance_id,
            effect_id,
            elapsed_rounds: advance.elapsed_rounds,
            remaining_duration: advance.remaining_duration,
        })
    }

    /// Updates or removes an instance after calculating its new duration.
    ///
    /// # Errors
    ///
    /// Returns an error when the instance is absent.
    fn update_duration(
        &mut self,
        instance_id: EffectInstanceId,
        duration: Option<EffectDuration>,
    ) -> CoreResult<()> {
        match duration {
            Some(duration) => {
                let effect = self.instances.get_mut(instance_id).ok_or_else(|| {
                    Box::new(CoreError::EffectInstanceNotFound { id: instance_id })
                })?;
                effect.duration = duration;
            }
            None => {
                self.remove(instance_id)?;
            }
        }

        Ok(())
    }

    /// Returns an instancfe by its runtime identifier.
    #[must_use]
    pub fn effect(&self, instance_id: EffectInstanceId) -> Option<&ActiveEffect> {
        self.instances.get(instance_id)
    }

    /// Returns whether at least one instance of the effect is active.
    #[must_use]
    pub fn is_active(&self, effect_id: &EffectId) -> bool {
        self.instances.is_active(effect_id)
    }

    /// Returns the number of active instances of an effect.
    #[must_use]
    pub fn instance_count(&self, effect_id: &EffectId) -> usize {
        self.instances.instance_count(effect_id)
    }

    /// Iterates over active instances of an effect.
    #[must_use]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &ActiveEffect> {
        self.instances.iter()
    }

    /// Returns the total number of active instances.
    #[must_use]
    pub fn len(&self) -> usize {
        self.instances.len()
    }

    /// Returns whether no effect instances are active.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }
}
