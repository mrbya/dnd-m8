//! Stable ruleset versioning

use semver::Version;

use crate::{RulesetError, RulesetResult};

/// Stable, versioned identity of one ruleset implementation.
///
/// Families should be globally namespaced, for instance
/// `org.dnd-m8.srd` or `com.example.campaign-rules`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RulesetId {
    /// Ruleset family namespace.
    family: String,
    /// Ruleset version.
    version: Version,
}

impl RulesetId {
    /// Constructs new ruleset ID.
    ///
    /// # Errors
    /// Returns [`RulesetError`] if:
    /// - ruleset family is empty or a whitespace,
    /// - provided ruleset version is invalid.
    pub fn new(family: impl Into<String>, version: impl AsRef<str>) -> RulesetResult<Self> {
        let family = family.into();
        let family = family.trim();

        if family.is_empty() {
            return Err(RulesetError::EmptyFamily);
        }

        let version =
            Version::parse(version.as_ref()).map_err(|error| RulesetError::InvalidVersion {
                error: error.to_string(),
            })?;

        Ok(Self {
            family: family.to_owned(),
            version,
        })
    }

    /// Returns globally namespaced ruleset family.
    #[must_use]
    pub fn family(&self) -> &str {
        &self.family
    }

    /// Returns ruleset version.
    #[must_use]
    pub const fn version(&self) -> &Version {
        &self.version
    }
}

impl std::fmt::Display for RulesetId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}@{}", self.family, self.version)
    }
}

/// Human-readable metadata describing an installed ruleset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RulesetMetadata {
    /// Stable identity of the ruleset.
    pub id: RulesetId,

    /// User-facing name.
    pub display_name: String,

    /// Short user-facing explanation.
    pub description: String,
}

#[cfg(test)]
mod tests {
    use crate::{RulesetError, RulesetId};

    #[test]
    fn ruleset_id_is_stable_and_versioned() {
        let id = RulesetId::new("org.dnd-m8.test", "1.2.3").expect("valid id");

        assert_eq!(id.family(), "org.dnd-m8.test");
        assert_eq!(id.to_string(), "org.dnd-m8.test@1.2.3");
    }

    #[test]
    fn ruleset_id_rejects_invalid_parts() {
        assert_eq!(RulesetId::new(" ", "1.0.0"), Err(RulesetError::EmptyFamily),);

        assert!(matches!(
            RulesetId::new("org.dnd-m8.test", "latest"),
            Err(RulesetError::InvalidVersion { .. }),
        ));
    }
}
