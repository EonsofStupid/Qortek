//! Shared primitives for the public Qortek contracts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, QortekError>;

#[derive(Debug, Error)]
pub enum QortekError {
    #[error("not found: {0}")]
    NotFound(String),
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
