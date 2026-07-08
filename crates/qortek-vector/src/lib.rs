//! Vector recall contracts for Qortex, Qdrant, and other recall engines.

use async_trait::async_trait;
use qortek_core::{Result, RroId, VectorId};
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait VectorRecall: Send + Sync {
    async fn upsert_vectors(&self, batch: VectorBatch) -> Result<Vec<VectorId>>;
    async fn search_dense(&self, query: DenseQuery) -> Result<Vec<VectorHit>>;
    async fn search_hybrid(&self, query: HybridQuery) -> Result<Vec<VectorHit>>;
    async fn mark_deleted(&self, id: VectorId) -> Result<()>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VectorBatch {
    pub collection: String,
    pub documents: Vec<VectorDocument>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VectorDocument {
    pub id: VectorId,
    pub source_rro: RroId,
    pub text: String,
    pub dense: Option<Vec<f32>>,
    pub sparse_terms: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DenseQuery {
    pub collection: String,
    pub vector: Vec<f32>,
    pub limit: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HybridQuery {
    pub collection: String,
    pub text: String,
    pub vector: Option<Vec<f32>>,
    pub limit: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VectorHit {
    pub id: VectorId,
    pub source_rro: RroId,
    pub score: f32,
    pub text: String,
}
