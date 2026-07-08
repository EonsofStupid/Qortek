//! In-memory BrainStore adapter for tests and examples.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use qortek_core::{EventId, ForkId, MemoryId, QortekError, Result, RroId};
use qortek_rro::ReasonReadyObject;
use qortek_store::{BrainEvent, BrainStore, FoldbackRecord, MemoryDeclaration, StoredFork};

#[derive(Clone, Default)]
pub struct InMemoryBrainStore {
    inner: Arc<RwLock<Inner>>,
}

#[derive(Default)]
struct Inner {
    rros: HashMap<RroId, ReasonReadyObject>,
    memories: HashMap<MemoryId, MemoryDeclaration>,
    forks: HashMap<ForkId, StoredFork>,
    foldbacks: HashMap<ForkId, FoldbackRecord>,
    events: HashMap<EventId, BrainEvent>,
}

#[async_trait]
impl BrainStore for InMemoryBrainStore {
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
        inner.memories.insert(id, memory);
        Ok(id)
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
