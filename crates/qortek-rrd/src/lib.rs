//! Daemon-facing orchestration shell for the Reason Ready Daemon.

use qortek_core::{Result, WorkspaceId};
use qortek_rro::{ReasonReadyObject, RroKind};
use qortek_store::{BrainEvent, BrainStore};
use serde_json::json;

pub struct ReasonReadyDaemon<B> {
    brain: B,
}

impl<B> ReasonReadyDaemon<B>
where
    B: BrainStore,
{
    #[must_use]
    pub fn new(brain: B) -> Self {
        Self { brain }
    }

    pub async fn start_seed_chat(
        &self,
        workspace_id: WorkspaceId,
        prompt: impl Into<String> + Send,
    ) -> Result<ReasonReadyObject> {
        let rro = ReasonReadyObject::new(workspace_id, RroKind::SeedChat, prompt.into());
        self.brain.put_rro(rro.clone()).await?;
        self.brain
            .append_event(BrainEvent::new(
                "seed_chat_started",
                json!({ "rro_id": rro.id.0.to_string() }),
            ))
            .await?;
        Ok(rro)
    }
}
