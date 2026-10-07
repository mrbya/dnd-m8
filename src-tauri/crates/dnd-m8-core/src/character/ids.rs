use core::fmt;
use std::marker::PhantomData;

use uuid::Uuid;

use crate::{CoreError, CoreResult};

/// Textual identity of a ruleset-defined item.
///
/// The marker distinguishes domains at compile time without storing a value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefinitionId<Kind> {
    /// Normalized ruleset-defined identifier.
    value: String,
    /// Compile-time domain marker.
    kind: PhantomData<fn() -> Kind>,
}

impl<Kind> DefinitionId<Kind> {
    /// Constructs an identifier using the domain's validation error.
    ///
    /// # Errors
    ///
    /// Returns `empty_error` when the trimmed identifier is empty.
    pub fn from_text(value: impl Into<String>, empty_error: CoreError) -> CoreResult<Self> {
        let value = value.into();
        let value = value.trim();

        if value.is_empty() {
            return Err(Box::new(empty_error));
        }

        Ok(Self {
            value: value.to_owned(),
            kind: PhantomData,
        })
    }

    /// Returns the normalized textual identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl<Kind> fmt::Display for DefinitionId<Kind> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

/// Unique identity of a runtime instance.
///
/// Instances from different domains have distinct types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstanceId<Kind> {
    /// Unique rintime identifier.
    value: Uuid,
    /// Compile-time domain marker.
    kind: PhantomData<fn() -> Kind>,
}

impl<Kind> InstanceId<Kind> {
    /// Generates a new runtime instance identifier.
    #[must_use]
    pub fn new() -> Self {
        Self::from_uuid(Uuid::new_v4())
    }

    /// Constructs an instance identifier from an existing UUID.
    #[must_use]
    pub fn from_uuid(value: Uuid) -> Self {
        Self {
            value,
            kind: PhantomData,
        }
    }

    /// Returns the underlying UUID.
    #[must_use]
    pub const fn as_uuid(&self) -> &Uuid {
        &self.value
    }
}

impl<Kind> Default for InstanceId<Kind> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Kind> fmt::Display for InstanceId<Kind> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
