//! Shared primitives for the public Qortek contracts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, QortekError>;

#[derive(Debug, Error)]
pub enum QortekError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("invalid state: {0}")]
    InvalidState(String),
    #[error("adapter error: {0}")]
    Adapter(String),
    #[error("serialization error: {0}")]
    Serialization(String),
}

macro_rules! id_type {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
        pub struct $name(pub Uuid);

        impl $name {
            #[must_use]
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

id_type!(RroId);
id_type!(MemoryId);
id_type!(ForkId);
id_type!(EventId);
id_type!(EdgeId);
id_type!(VectorId);
id_type!(NodeId);
id_type!(WorkspaceId);

/// Public project routing key.
///
/// This mirrors the platform_devpulse IntelligenceStore `slug` concept:
/// every memory, archive, relation, and recall operation must resolve
/// against an explicit project scope instead of a global pile of data.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ProjectSlug(String);

impl ProjectSlug {
    /// Create a validated project slug.
    ///
    /// Allowed characters are ASCII lowercase letters, digits, `_`, and `-`.
    /// This keeps slugs safe for adapters that map them to namespaces,
    /// database names, collection names, or filesystem paths.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.is_empty() {
            return Err(QortekError::InvalidInput("project slug cannot be empty".into()));
        }
        if value.len() > 96 {
            return Err(QortekError::InvalidInput(
                "project slug cannot exceed 96 characters".into(),
            ));
        }
        if !value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
        {
            return Err(QortekError::InvalidInput(format!(
                "invalid project slug `{value}`; use lowercase ascii, digits, `_`, or `-`"
            )));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProjectSlug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Explicit namespace for all project-scoped brain operations.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ProjectScope {
    pub workspace_id: WorkspaceId,
    pub slug: ProjectSlug,
}

impl ProjectScope {
    #[must_use]
    pub fn new(workspace_id: WorkspaceId, slug: ProjectSlug) -> Self {
        Self { workspace_id, slug }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Timestamped<T> {
    pub value: T,
    pub created_at: DateTime<Utc>,
}

impl<T> Timestamped<T> {
    #[must_use]
    pub fn now(value: T) -> Self {
        Self {
            value,
            created_at: Utc::now(),
        }
    }
}
