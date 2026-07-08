//! Durable brain-store contracts.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use qortek_core::{EventId, ForkId, MemoryId, Result, RroId};
use qortek_rro::ReasonReadyObject;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait BrainStore: Send + Sync {
    async fn put_rro(&self, rro: ReasonReadyObject) -> Result<RroId>;
    async fn get_rro(&self, id: RroId) -> Result<Option<ReasonReadyObject>>;
    async fn declare_memory(&self, memory: MemoryDeclaration) -> Result<MemoryId>;
    async fn open_fork(&self, fork: StoredFork) -> Result<ForkId>;
    async fn fold_fork(&self, fork_id: ForkId, foldback: FoldbackRecord) -> Result<()>;
    async fn append_event(&self, event: BrainEvent) -> Result<EventId>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryDeclaration {
    pub id: MemoryId,
    pub source_rro: RroId,
    pub text: String,
    pub declared_at: DateTime<Utc>,
}

impl MemoryDeclaration {
    #[must_use]
    pub fn new(source_rro: RroId, text: impl Into<String>) -> Self {
        Self {
            id: MemoryId::new(),
            source_rro,
            text: text.into(),
            declared_at: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredFork {
    pub id: ForkId,
    pub seed_rro: RroId,
    pub topic: String,
    pub opened_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FoldbackRecord {
    pub summary: String,
    pub committed_rros: Vec<RroId>,
    pub folded_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BrainEvent {
    pub id: EventId,
    pub kind: String,
    pub payload_json: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

impl BrainEvent {
    #[must_use]
    pub fn new(kind: impl Into<String>, payload_json: serde_json::Value) -> Self {
        Self {
            id: EventId::new(),
            kind: kind.into(),
            payload_json,
            created_at: Utc::now(),
        }
    }
}
