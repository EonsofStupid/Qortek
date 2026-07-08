//! In-memory BrainStore adapter for tests, examples, and local replay.
//!
//! This adapter is not the final production store. It proves the public
//! BrainStore contract without requiring private qortex, ClyffyOS, SurrealDB,
//! Qdrant, or external services.

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use qortek_core::{EdgeId, EventId, ForkId, MemoryId, ProjectScope, QortekError, Result, RroId};
use qortek_rro::ReasonReadyObject;
use qortek_store::{
    AlignmentJournalEntry, ArchiveBlob, BrainEvent, BrainStore, FoldbackRecord,
    MemoryDeclaration, MemoryKind, MemoryRecord, ReasonEdge, RecallHit, StoredFork,
};

#[derive(Clone, Default)]
pub struct InMemoryBrainStore {
    inner: Arc<RwLock<Inner>>,
}

#[derive(Default)]
struct Inner {
    projects: HashSet<ProjectScope>,
    rros: HashMap<RroId, ReasonReadyObject>,
    declarations: HashMap<MemoryId, MemoryDeclaration>,
    memories: HashMap<(ProjectScope, MemoryId), MemoryRecord>,
    edges: HashMap<(ProjectScope, EdgeId), ReasonEdge>,
    archives: HashMap<(ProjectScope, MemoryId), ArchiveBlob>,
    alignments: Vec<AlignmentJournalEntry>,
    forks: HashMap<ForkId, StoredFork>,
    foldbacks: HashMap<ForkId, FoldbackRecord>,
    events: HashMap<EventId, BrainEvent>,
}

#[async_trait]
impl BrainStore for InMemoryBrainStore {
    async fn init_project(&self, scope: &ProjectScope) -> Result<()> {
        let mut inner = self.write()?;
        inner.projects.insert(scope.clone());
        Ok(())
    }

    async fn put_rro(&self, rro: ReasonReadyObject) -> Result<RroId> {
        let id = rro.id;
        let mut inner = self.write()?;
        inner.rros.insert(id, rro);
        Ok(id)
    }

    async fn get_rro(&self, id: RroId) -> Result<Option<ReasonReadyObject>> {
        let inner = self.read()?;
        Ok(inner.rros.get(&id).cloned())
    }

    async fn declare_memory(&self, memory: MemoryDeclaration) -> Result<MemoryId> {
        let id = memory.id;
        let mut inner = self.write()?;
        inner.declarations.insert(id, memory);
        Ok(id)
    }

    async fn save_memory(&self, scope: &ProjectScope, record: MemoryRecord) -> Result<MemoryId> {
        self.ensure_project(scope)?;
        let id = record.id;
        let mut inner = self.write()?;
        inner.memories.insert((scope.clone(), id), record);
        Ok(id)
    }

    async fn get_memory(
        &self,
        scope: &ProjectScope,
        id: MemoryId,
    ) -> Result<Option<MemoryRecord>> {
        self.ensure_project(scope)?;
        let inner = self.read()?;
        Ok(inner.memories.get(&(scope.clone(), id)).cloned())
    }

    async fn recall_by_embedding(
        &self,
        scope: &ProjectScope,
        query_embedding: &[f32],
        top_k: usize,
        kind: Option<MemoryKind>,
    ) -> Result<Vec<RecallHit>> {
        self.ensure_project(scope)?;
        if top_k == 0 {
            return Ok(Vec::new());
        }

        let inner = self.read()?;
        let mut hits = inner
            .memories
            .iter()
            .filter_map(|((memory_scope, _), record)| {
                if memory_scope != scope {
                    return None;
                }
                if kind.as_ref().is_some_and(|expected| expected != &record.kind) {
                    return None;
                }
                cosine_similarity(query_embedding, &record.embedding).map(|score| RecallHit {
                    record: record.clone(),
                    score,
                })
            })
            .collect::<Vec<_>>();

        hits.sort_by(|left, right| {
            right
                .score
                .partial_cmp(&left.score)
                .unwrap_or(Ordering::Equal)
        });
        hits.truncate(top_k);
        Ok(hits)
    }

    async fn relate(&self, scope: &ProjectScope, edge: ReasonEdge) -> Result<EdgeId> {
        self.ensure_project(scope)?;
        let id = edge.id;
        let mut inner = self.write()?;
        inner.edges.insert((scope.clone(), id), edge);
        Ok(id)
    }

    async fn edges_for(&self, scope: &ProjectScope, memory_id: MemoryId) -> Result<Vec<ReasonEdge>> {
        self.ensure_project(scope)?;
        let inner = self.read()?;
        Ok(inner
            .edges
            .iter()
            .filter_map(|((edge_scope, _), edge)| {
                if edge_scope == scope && (edge.from == memory_id || edge.to == memory_id) {
                    Some(edge.clone())
                } else {
                    None
                }
            })
            .collect())
    }

    async fn write_archive(&self, scope: &ProjectScope, archive: ArchiveBlob) -> Result<()> {
        self.ensure_project(scope)?;
        let mut inner = self.write()?;
        inner
            .archives
            .insert((scope.clone(), archive.memory_id), archive);
        Ok(())
    }

    async fn read_archive_bytes(
        &self,
        scope: &ProjectScope,
        memory_id: MemoryId,
    ) -> Result<Option<Vec<u8>>> {
        self.ensure_project(scope)?;
        let inner = self.read()?;
        Ok(inner
            .archives
            .get(&(scope.clone(), memory_id))
            .map(|archive| archive.bytes.clone()))
    }

    async fn record_alignment(&self, entry: AlignmentJournalEntry) -> Result<()> {
        let mut inner = self.write()?;
        inner.alignments.push(entry);
        Ok(())
    }

    async fn alignment_for_rro(&self, source_rro: RroId) -> Result<Vec<AlignmentJournalEntry>> {
        let inner = self.read()?;
        Ok(inner
            .alignments
            .iter()
            .filter(|entry| entry.source_rro == source_rro)
            .cloned()
            .collect())
    }

    async fn open_fork(&self, fork: StoredFork) -> Result<ForkId> {
        let id = fork.id;
        let mut inner = self.write()?;
        inner.forks.insert(id, fork);
        Ok(id)
    }

    async fn fold_fork(&self, fork_id: ForkId, foldback: FoldbackRecord) -> Result<()> {
        let mut inner = self.write()?;
        if !inner.forks.contains_key(&fork_id) {
            return Err(QortekError::NotFound(format!("fork {fork_id:?}")));
        }
        inner.foldbacks.insert(fork_id, foldback);
        Ok(())
    }

    async fn append_event(&self, event: BrainEvent) -> Result<EventId> {
        let id = event.id;
        let mut inner = self.write()?;
        inner.events.insert(id, event);
        Ok(id)
    }
}

impl InMemoryBrainStore {
    fn ensure_project(&self, scope: &ProjectScope) -> Result<()> {
        let inner = self.read()?;
        if inner.projects.contains(scope) {
            Ok(())
        } else {
            Err(QortekError::NotFound(format!(
                "project scope {}",
                scope.slug
            )))
        }
    }

    fn read(&self) -> Result<std::sync::RwLockReadGuard<'_, Inner>> {
        self.inner
            .read()
            .map_err(|_| QortekError::InvalidState("memory store lock poisoned".into()))
    }

    fn write(&self) -> Result<std::sync::RwLockWriteGuard<'_, Inner>> {
        self.inner
            .write()
            .map_err(|_| QortekError::InvalidState("memory store lock poisoned".into()))
    }
}

fn cosine_similarity(left: &[f32], right: &[f32]) -> Option<f32> {
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
