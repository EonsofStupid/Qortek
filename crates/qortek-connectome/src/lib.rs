//! Connectome orchestration layer.
//!
//! Connectome owns the meaning layer: project scope, durable memory,
//! RRO lineage, graph relationships, alignment journal, and resolved recall.
//! TotalRecall owns fast vector recall underneath it.

use async_trait::async_trait;
use qortek_core::{MemoryId, ProjectScope, Result, RroId, VectorId};
use qortek_rro::{ReasonReadyObject, RroKind};
use qortek_store::{
    AlignmentJournalEntry, ArchiveBlob, BrainStore, MemoryKind, MemoryRecord, ReasonEdge,
    RecallHit, VectorAlignmentStatus,
};
use qortek_vector::{DenseQuery, TotalRecall, VectorBatch, VectorDocument, VectorHit};
use serde::{Deserialize, Serialize};

pub const DEFAULT_COLLECTION: &str = "connectome-memory";

#[derive(Clone)]
pub struct Connectome<B, T> {
    brain: B,
    recall: T,
    collection: String,
}

impl<B, T> Connectome<B, T>
where
    B: BrainStore,
    T: TotalRecall,
{
    #[must_use]
    pub fn new(brain: B, recall: T) -> Self {
        Self {
            brain,
            recall,
            collection: DEFAULT_COLLECTION.to_string(),
        }
    }

    #[must_use]
    pub fn with_collection(mut self, collection: impl Into<String>) -> Self {
        self.collection = collection.into();
        self
    }

    pub async fn init_project(&self, scope: &ProjectScope) -> Result<()> {
        self.brain.init_project(scope).await
    }

    pub async fn ingest_memory(
        &self,
        request: ConnectomeIngestRequest,
    ) -> Result<ConnectomeIngestReport> {
        let seed = ReasonReadyObject::new(
            request.scope.workspace_id,
            RroKind::Memory,
            request.seed_text.clone(),
        );
        let source_rro = self.brain.put_rro(seed).await?;

        let memory = MemoryRecord::new(
            source_rro,
            request.memory_text.clone(),
            request.embedding.clone(),
            request.kind,
        );
        let memory_id = memory.id;

        self.brain.save_memory(&request.scope, memory).await?;
        self.brain
            .record_alignment(AlignmentJournalEntry::new(
                source_rro,
                Some(memory_id),
                None,
                VectorAlignmentStatus::PendingVectorization,
                1,
            ))
            .await?;

        let vector_id = VectorId::new();
        let upserted = self
            .recall
            .upsert_vectors(VectorBatch {
                collection: self.collection.clone(),
                scope: request.scope.clone(),
                documents: vec![VectorDocument {
                    id: vector_id,
                    source_rro,
                    memory_id,
                    text: request.memory_text,
                    dense: Some(request.embedding),
                    sparse_terms: request.sparse_terms,
                }],
            })
            .await?;

        let confirmed_vector_id = upserted.first().copied().unwrap_or(vector_id);
        self.brain
            .record_alignment(AlignmentJournalEntry::new(
                source_rro,
                Some(memory_id),
                Some(confirmed_vector_id),
                VectorAlignmentStatus::Vectorized,
                2,
            ))
            .await?;

        if let Some(bytes) = request.archive_bytes {
            self.brain
                .write_archive(
                    &request.scope,
                    ArchiveBlob::new(memory_id, source_rro, bytes, request.archive_codec),
                )
                .await?;
        }

        Ok(ConnectomeIngestReport {
            source_rro,
            memory_id,
            vector_id: confirmed_vector_id,
        })
    }

    pub async fn recall(&self, request: ConnectomeRecallRequest) -> Result<Vec<ResolvedRecall>> {
        let hits = self
            .recall
            .search_dense(DenseQuery {
                collection: self.collection.clone(),
                scope: request.scope.clone(),
                vector: request.embedding,
                limit: request.limit,
            })
            .await?;

        self.resolve_hits(&request.scope, hits).await
    }

    pub async fn relate(
        &self,
        scope: &ProjectScope,
        from: MemoryId,
        to: MemoryId,
        relation: impl Into<String>,
    ) -> Result<qortek_core::EdgeId> {
        self.brain
            .relate(scope, ReasonEdge::new(from, to, relation))
            .await
    }

    async fn resolve_hits(
        &self,
        scope: &ProjectScope,
        hits: Vec<VectorHit>,
    ) -> Result<Vec<ResolvedRecall>> {
        let mut resolved = Vec::with_capacity(hits.len());

        for hit in hits {
            if let Some(memory) = self.brain.get_memory(scope, hit.memory_id).await? {
                let edges = self.brain.edges_for(scope, hit.memory_id).await?;
                let alignment = self.brain.alignment_for_rro(hit.source_rro).await?;
                resolved.push(ResolvedRecall {
                    memory,
                    vector_id: hit.id,
                    score: hit.score,
                    edges,
                    alignment,
                });
            }
        }

        Ok(resolved)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConnectomeIngestRequest {
    pub scope: ProjectScope,
    pub seed_text: String,
    pub memory_text: String,
    pub embedding: Vec<f32>,
    pub sparse_terms: Vec<String>,
    pub kind: MemoryKind,
    pub archive_bytes: Option<Vec<u8>>,
    pub archive_codec: String,
}

impl ConnectomeIngestRequest {
    #[must_use]
    pub fn decision(scope: ProjectScope, text: impl Into<String>, embedding: Vec<f32>) -> Self {
        let text = text.into();
        Self {
            scope,
            seed_text: text.clone(),
            memory_text: text,
            embedding,
            sparse_terms: Vec::new(),
            kind: MemoryKind::Decision,
            archive_bytes: None,
            archive_codec: "raw".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConnectomeIngestReport {
    pub source_rro: RroId,
    pub memory_id: MemoryId,
    pub vector_id: VectorId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConnectomeRecallRequest {
    pub scope: ProjectScope,
    pub embedding: Vec<f32>,
    pub limit: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResolvedRecall {
    pub memory: MemoryRecord,
    pub vector_id: VectorId,
    pub score: f32,
    pub edges: Vec<ReasonEdge>,
    pub alignment: Vec<AlignmentJournalEntry>,
}

#[async_trait]
pub trait IntentClassifier: Send + Sync {
    async fn classify(&self, raw_input: &str) -> Result<ClassifiedIntent>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassifiedIntent {
    pub raw_input: String,
    pub kind: IntentKind,
    pub confidence: f32,
    pub requires_project_selection: bool,
    pub security_tags: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum IntentKind {
    Fix,
    Build,
    Explain,
    Plan,
    Unknown,
}

#[must_use]
pub fn recall_hit_to_brain_hit(hit: &ResolvedRecall) -> RecallHit {
    RecallHit {
        record: hit.memory.clone(),
        score: hit.score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qortek_adapter_memory::InMemoryBrainStore;
    use qortek_core::{ProjectSlug, QortekError, WorkspaceId};
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::sync::{Arc, RwLock};

    #[derive(Clone, Default)]
    struct InMemoryTotalRecall {
        docs: Arc<RwLock<HashMap<VectorId, VectorDocument>>>,
    }

    #[async_trait]
    impl TotalRecall for InMemoryTotalRecall {
        async fn upsert_vectors(&self, batch: VectorBatch) -> Result<Vec<VectorId>> {
            let mut docs = self
                .docs
                .write()
                .map_err(|_| QortekError::InvalidState("recall lock poisoned".into()))?;
            let ids = batch.documents.iter().map(|doc| doc.id).collect::<Vec<_>>();
            for doc in batch.documents {
                docs.insert(doc.id, doc);
            }
            Ok(ids)
        }

        async fn search_dense(&self, query: DenseQuery) -> Result<Vec<VectorHit>> {
            let docs = self
                .docs
                .read()
                .map_err(|_| QortekError::InvalidState("recall lock poisoned".into()))?;
            let mut hits = docs
                .values()
                .filter_map(|doc| {
                    cosine(&query.vector, doc.dense.as_ref()?).map(|score| VectorHit {
                        id: doc.id,
                        source_rro: doc.source_rro,
                        memory_id: doc.memory_id,
                        score,
                        text: doc.text.clone(),
                    })
                })
                .collect::<Vec<_>>();
            hits.sort_by(|left, right| {
                right
                    .score
                    .partial_cmp(&left.score)
                    .unwrap_or(Ordering::Equal)
            });
            hits.truncate(query.limit);
            Ok(hits)
        }

        async fn search_hybrid(
            &self,
            query: qortek_vector::HybridQuery,
        ) -> Result<Vec<VectorHit>> {
            match query.vector {
                Some(vector) => {
                    self.search_dense(DenseQuery {
                        collection: query.collection,
                        scope: query.scope,
                        vector,
                        limit: query.limit,
                    })
                    .await
                }
                None => Ok(Vec::new()),
            }
        }

        async fn mark_deleted(&self, id: VectorId) -> Result<()> {
            let mut docs = self
                .docs
                .write()
                .map_err(|_| QortekError::InvalidState("recall lock poisoned".into()))?;
            docs.remove(&id);
            Ok(())
        }
    }

    fn cosine(left: &[f32], right: &[f32]) -> Option<f32> {
        if left.is_empty() || left.len() != right.len() {
            return None;
        }
        let mut dot = 0.0_f32;
        let mut left_norm = 0.0_f32;
        let mut right_norm = 0.0_f32;
        for (l, r) in left.iter().zip(right.iter()) {
            dot += l * r;
            left_norm += l * l;
            right_norm += r * r;
        }
        if left_norm == 0.0 || right_norm == 0.0 {
            return None;
        }
        Some(dot / (left_norm.sqrt() * right_norm.sqrt()))
    }

    #[tokio::test]
    async fn connectome_ingests_indexes_and_resolves_recall() {
        let brain = InMemoryBrainStore::default();
        let recall = InMemoryTotalRecall::default();
        let connectome = Connectome::new(brain, recall);
        let scope = ProjectScope::new(
            WorkspaceId::new(),
            ProjectSlug::new("operator-estate").expect("valid slug"),
        );
        connectome.init_project(&scope).await.expect("init project");

        let report = connectome
            .ingest_memory(ConnectomeIngestRequest {
                scope: scope.clone(),
                seed_text: "TotalRecall indexes derived vectors.".to_string(),
                memory_text: "Connectome owns meaning; TotalRecall owns fast derived recall."
                    .to_string(),
                embedding: vec![1.0, 0.0, 0.0, 0.0],
                sparse_terms: vec!["connectome".to_string(), "totalrecall".to_string()],
                kind: MemoryKind::Decision,
                archive_bytes: Some(b"rro-intent-tags-security-years".to_vec()),
                archive_codec: "raw-test".to_string(),
            })
            .await
            .expect("ingest memory");

        let results = connectome
            .recall(ConnectomeRecallRequest {
                scope,
                embedding: vec![1.0, 0.0, 0.0, 0.0],
                limit: 4,
            })
            .await
            .expect("recall");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].memory.id, report.memory_id);
        assert_eq!(results[0].vector_id, report.vector_id);
        assert!(results[0].score > 0.99);
        assert_eq!(results[0].alignment.len(), 2);
        assert_eq!(
            results[0].alignment.last().expect("alignment").status,
            VectorAlignmentStatus::Vectorized
        );
    }
}
