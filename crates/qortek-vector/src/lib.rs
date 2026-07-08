//! TotalRecall contracts.
//!
//! TotalRecall is the public name for the forked-Qdrant recall engine track.
//! This crate defines the adapter-facing port only; the private engine plugs
//! in behind this trait without leaking private source into Qortek.

use async_trait::async_trait;
use qortek_core::{MemoryId, ProjectScope, Result, RroId, VectorId};
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait TotalRecall: Send + Sync {
    async fn upsert_vectors(&self, batch: VectorBatch) -> Result<Vec<VectorId>>;
    async fn search_dense(&self, query: DenseQuery) -> Result<Vec<VectorHit>>;
    async fn search_hybrid(&self, query: HybridQuery) -> Result<Vec<VectorHit>>;
    async fn mark_deleted(&self, id: VectorId) -> Result<()>;
}

/// Compatibility alias for older planning/code that used `VectorRecall`.
/// New code should use [`TotalRecall`].
#[async_trait]
pub trait VectorRecall: TotalRecall {}

impl<T> VectorRecall for T where T: TotalRecall + ?Sized {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VectorBatch {
    pub collection: String,
    pub scope: ProjectScope,
    pub documents: Vec<VectorDocument>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VectorDocument {
    pub id: VectorId,
    pub source_rro: RroId,
    pub memory_id: MemoryId,
    pub text: String,
    pub dense: Option<Vec<f32>>,
    pub sparse_terms: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DenseQuery {
    pub collection: String,
    pub scope: ProjectScope,
    pub vector: Vec<f32>,
    pub limit: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HybridQuery {
    pub collection: String,
    pub scope: ProjectScope,
    pub text: String,
    pub vector: Option<Vec<f32>>,
    pub limit: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VectorHit {
    pub id: VectorId,
    pub source_rro: RroId,
    pub memory_id: MemoryId,
    pub score: f32,
    pub text: String,
}
