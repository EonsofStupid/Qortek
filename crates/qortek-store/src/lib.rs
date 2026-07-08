//! Durable brain-store contracts.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use qortek_core::{
    EdgeId, EventId, ForkId, MemoryId, ProjectScope, Result, RroId, VectorId,
};
use qortek_rro::ReasonReadyObject;
use serde::{Deserialize, Serialize};

/// Durable source of truth for Qortek.
///
/// BrainStore owns persistent truth. Vector indexes are derived from it and
/// repairable through alignment journal state.
#[async_trait]
pub trait BrainStore: Send + Sync {
    async fn init_project(&self, scope: &ProjectScope) -> Result<()>;

    async fn put_rro(&self, rro: ReasonReadyObject) -> Result<RroId>;
    async fn get_rro(&self, id: RroId) -> Result<Option<ReasonReadyObject>>;

    async fn declare_memory(&self, memory: MemoryDeclaration) -> Result<MemoryId>;
    async fn save_memory(&self, scope: &ProjectScope, record: MemoryRecord) -> Result<MemoryId>;
    async fn get_memory(
        &self,
        scope: &ProjectScope,
        id: MemoryId,
    ) -> Result<Option<MemoryRecord>>;
    async fn recall_by_embedding(
        &self,
        scope: &ProjectScope,
        query_embedding: &[f32],
        top_k: usize,
        kind: Option<MemoryKind>,
    ) -> Result<Vec<RecallHit>>;

    async fn relate(&self, scope: &ProjectScope, edge: ReasonEdge) -> Result<EdgeId>;
    async fn edges_for(&self, scope: &ProjectScope, memory_id: MemoryId) -> Result<Vec<ReasonEdge>>;

    async fn write_archive(&self, scope: &ProjectScope, archive: ArchiveBlob) -> Result<()>;
    async fn read_archive_bytes(
        &self,
        scope: &ProjectScope,
        memory_id: MemoryId,
    ) -> Result<Option<Vec<u8>>>;

    async fn record_alignment(&self, entry: AlignmentJournalEntry) -> Result<()>;
    async fn alignment_for_rro(&self, source_rro: RroId) -> Result<Vec<AlignmentJournalEntry>>;

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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MemoryKind {
    Operator,
    Project,
    Fact,
    Decision,
    ToolTrace,
    ForkSummary,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub id: MemoryId,
    pub source_rro: RroId,
    pub text: String,
    pub embedding: Vec<f32>,
    pub kind: MemoryKind,
    pub priority: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl MemoryRecord {
    #[must_use]
    pub fn new(
        source_rro: RroId,
        text: impl Into<String>,
        embedding: Vec<f32>,
        kind: MemoryKind,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: MemoryId::new(),
            source_rro,
            text: text.into(),
            embedding,
            kind,
            priority: 5,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecallHit {
    pub record: MemoryRecord,
    pub score: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReasonEdge {
    pub id: EdgeId,
    pub from: MemoryId,
    pub to: MemoryId,
    pub relation: String,
    pub created_at: DateTime<Utc>,
}

impl ReasonEdge {
    #[must_use]
    pub fn new(from: MemoryId, to: MemoryId, relation: impl Into<String>) -> Self {
        Self {
            id: EdgeId::new(),
            from,
            to,
            relation: relation.into(),
            created_at: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArchiveBlob {
    pub memory_id: MemoryId,
    pub source_rro: RroId,
    pub bytes: Vec<u8>,
    pub codec: String,
    pub created_at: DateTime<Utc>,
}

impl ArchiveBlob {
    #[must_use]
    pub fn new(memory_id: MemoryId, source_rro: RroId, bytes: Vec<u8>, codec: impl Into<String>) -> Self {
        Self {
            memory_id,
            source_rro,
            bytes,
            codec: codec.into(),
            created_at: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum VectorAlignmentStatus {
    PendingVectorization,
    Vectorized,
    VectorOutOfDate,
    VectorDeleted,
    RepairNeeded,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlignmentJournalEntry {
    pub source_rro: RroId,
    pub memory_id: Option<MemoryId>,
    pub vector_id: Option<VectorId>,
    pub status: VectorAlignmentStatus,
    pub version: u64,
    pub note: Option<String>,
    pub recorded_at: DateTime<Utc>,
}

impl AlignmentJournalEntry {
    #[must_use]
    pub fn new(
        source_rro: RroId,
        memory_id: Option<MemoryId>,
        vector_id: Option<VectorId>,
        status: VectorAlignmentStatus,
        version: u64,
    ) -> Self {
        Self {
            source_rro,
            memory_id,
            vector_id,
            status,
            version,
            note: None,
            recorded_at: Utc::now(),
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
