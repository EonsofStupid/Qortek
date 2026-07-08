//! Reason Ready Object model.

use chrono::{DateTime, Utc};
use qortek_core::{EdgeId, RroId, VectorId, WorkspaceId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReasonReadyObject {
    pub id: RroId,
    pub workspace_id: WorkspaceId,
    pub kind: RroKind,
    pub seed: SeedContext,
    pub claims: Vec<Claim>,
    pub evidence: Vec<EvidenceRef>,
    pub boundaries: Vec<BoundaryRef>,
    pub graph_edges: Vec<EdgeRef>,
    pub vector_refs: Vec<VectorRef>,
    pub lifecycle: RroLifecycle,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ReasonReadyObject {
    #[must_use]
    pub fn new(workspace_id: WorkspaceId, kind: RroKind, seed: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: RroId::new(),
            workspace_id,
            kind,
            seed: SeedContext { text: seed.into() },
            claims: Vec::new(),
            evidence: Vec::new(),
            boundaries: Vec::new(),
            graph_edges: Vec::new(),
            vector_refs: Vec::new(),
            lifecycle: RroLifecycle::Draft,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RroKind {
    SeedChat,
    Memory,
    Plan,
    Decision,
    ToolTrace,
    ForkSummary,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeedContext {
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claim {
    pub text: String,
    pub confidence: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvidenceRef {
    pub uri: String,
    pub note: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoundaryRef {
    pub name: String,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EdgeRef {
    pub id: EdgeId,
    pub relation: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VectorRef {
    pub id: VectorId,
    pub collection: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RroLifecycle {
    Draft,
    Declared,
    VectorPending,
    Vectorized,
    Superseded,
    Archived,
}
