use qortek_adapter_memory::InMemoryBrainStore;
use qortek_core::{ProjectScope, ProjectSlug, VectorId, WorkspaceId};
use qortek_rro::{ReasonReadyObject, RroKind};
use qortek_store::{
    AlignmentJournalEntry, ArchiveBlob, BrainStore, BrainEvent, MemoryKind, MemoryRecord,
    ReasonEdge, VectorAlignmentStatus,
};
use serde_json::json;

#[tokio::main]
async fn main() -> qortek_core::Result<()> {
    let brain = InMemoryBrainStore::default();
    let scope = ProjectScope::new(
        WorkspaceId::new(),
        ProjectSlug::new("operator-estate")?,
    );

    brain.init_project(&scope).await?;

    let seed = ReasonReadyObject::new(
        scope.workspace_id,
        RroKind::SeedChat,
        "Operator establishes a local estate memory moat.",
    );
    brain.put_rro(seed.clone()).await?;

    let durable_truth = MemoryRecord::new(
        seed.id,
        "Connectome owns durable truth: estate memory, graph edges, replay, and alignment journal.",
        vec![1.0, 0.0, 0.0, 0.0],
        MemoryKind::Decision,
    );
    let durable_truth_id = brain.save_memory(&scope, durable_truth).await?;

    let recall_truth = MemoryRecord::new(
        seed.id,
        "Qortex owns recall indexes, which are fast, derived, and repairable.",
        vec![0.86, 0.14, 0.0, 0.0],
        MemoryKind::Fact,
    );
    let recall_truth_id = brain.save_memory(&scope, recall_truth).await?;

    let edge = ReasonEdge::new(durable_truth_id, recall_truth_id, "aligns_with");
    brain.relate(&scope, edge).await?;

    brain
        .write_archive(
            &scope,
            ArchiveBlob::new(
                durable_truth_id,
                seed.id,
                b"reason-ready-context:connectome-qortex-harmony".to_vec(),
                "raw-demo",
            ),
        )
        .await?;

    let vector_id = VectorId::new();
    brain
        .record_alignment(AlignmentJournalEntry::new(
            seed.id,
            Some(durable_truth_id),
            Some(vector_id),
            VectorAlignmentStatus::Vectorized,
            1,
        ))
        .await?;

    brain
        .append_event(BrainEvent::new(
            "replay_memory_moat_completed",
            json!({
                "project": scope.slug.as_str(),
                "seed_rro": seed.id.0.to_string(),
                "durable_memory": durable_truth_id.0.to_string(),
                "recall_memory": recall_truth_id.0.to_string(),
                "vector_id": vector_id.0.to_string()
            }),
        ))
        .await?;

    let hits = brain
        .recall_by_embedding(&scope, &[1.0, 0.0, 0.0, 0.0], 8, None)
        .await?;
    let edges = brain.edges_for(&scope, durable_truth_id).await?;
    let archive = brain
        .read_archive_bytes(&scope, durable_truth_id)
        .await?
        .unwrap_or_default();
    let alignments = brain.alignment_for_rro(seed.id).await?;

    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "replay": "memory-moat",
            "project": scope.slug.as_str(),
            "seed_rro": seed.id.0.to_string(),
            "recall_hits": hits.iter().map(|hit| json!({
                "memory_id": hit.record.id.0.to_string(),
                "kind": format!("{:?}", hit.record.kind),
                "score": hit.score,
                "text": hit.record.text
            })).collect::<Vec<_>>(),
            "edges": edges.iter().map(|edge| json!({
                "edge_id": edge.id.0.to_string(),
                "from": edge.from.0.to_string(),
                "to": edge.to.0.to_string(),
                "relation": edge.relation
            })).collect::<Vec<_>>(),
            "archive_bytes": String::from_utf8_lossy(&archive),
            "alignments": alignments.iter().map(|entry| json!({
                "source_rro": entry.source_rro.0.to_string(),
                "memory_id": entry.memory_id.map(|id| id.0.to_string()),
                "vector_id": entry.vector_id.map(|id| id.0.to_string()),
                "status": format!("{:?}", entry.status),
                "version": entry.version
            })).collect::<Vec<_>>()
        }))
        .map_err(|err| qortek_core::QortekError::Serialization(err.to_string()))?
    );

    Ok(())
}
