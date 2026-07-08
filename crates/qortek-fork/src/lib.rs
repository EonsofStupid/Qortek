//! Fork / deliberate / commit / foldback model.

use chrono::{DateTime, Utc};
use qortek_core::{ForkId, RroId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConversationFork {
    pub id: ForkId,
    pub seed_rro: RroId,
    pub topic: String,
    pub status: ForkStatus,
    pub opened_at: DateTime<Utc>,
    pub committed_at: Option<DateTime<Utc>>,
}

impl ConversationFork {
    #[must_use]
    pub fn open(seed_rro: RroId, topic: impl Into<String>) -> Self {
        Self {
            id: ForkId::new(),
            seed_rro,
            topic: topic.into(),
            status: ForkStatus::Open,
            opened_at: Utc::now(),
            committed_at: None,
        }
    }

    #[must_use]
    pub fn commit(mut self) -> Self {
        self.status = ForkStatus::Committed;
        self.committed_at = Some(Utc::now());
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ForkStatus {
    Open,
    Deliberating,
    CommitReady,
    Committed,
    FoldedBack,
    Abandoned,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Foldback {
    pub fork_id: ForkId,
    pub summary: String,
    pub committed_rros: Vec<RroId>,
    pub folded_at: DateTime<Utc>,
}

impl Foldback {
    #[must_use]
    pub fn new(fork_id: ForkId, summary: impl Into<String>, committed_rros: Vec<RroId>) -> Self {
        Self {
            fork_id,
            summary: summary.into(),
            committed_rros,
            folded_at: Utc::now(),
        }
    }
}
