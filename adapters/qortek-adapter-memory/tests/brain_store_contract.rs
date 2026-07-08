use qortek_adapter_memory::InMemoryBrainStore;
use qortek_core::{ProjectScope, ProjectSlug, VectorId, WorkspaceId};
use qortek_rro::{ReasonReadyObject, RroKind};
use qortek_store::{
    AlignmentJournalEntry, ArchiveBlob, BrainStore, MemoryKind, MemoryRecord, ReasonEdge,
    VectorAlignmentStatus,
};

fn test_scope() -> ProjectScope {
    ProjectScope::new(
        WorkspaceId::new(),
        ProjectSlug::new("test-project").expect("valid slug"),
    )
}

#[tokio::test]
async fn memory_adapter_saves_recalls_relates_archives_and_tracks_alignment() {
    let store = InMemoryBrainStore::default();
    let scope = test_scope();
    store.init_project(&scope).await.expect("project init");

    let seed = ReasonReadyObject::new(
        scope.workspace_id,
        RroKind::SeedChat,
        "Build a local memory moat for the operator.",
    );
    store.put_rro(seed.clone()).await.expect("put rro");

    let memory = MemoryRecord::new(
        seed.id,
        "Qortek stores durable brain truth and Qortex stores rebuildable recall indexes.",
        vec![1.0, 0.0, 0.0, 0.0],
        MemoryKind::Decision,
    );
    let memory_id = store
        .save_memory(&scope, memory.clone())
        .await
        .expect("save memory");

    let fetched = store
        .get_memory(&scope, memory_id)
        .await
        .expect("get memory")
        .expect("memory exists");
    assert_eq!(fetched.text, memory.text);

    let hits = store
        .recall_by_embedding(&scope, &[1.0, 0.0, 0.0, 0.0], 3, Some(MemoryKind::Decision))
        .await
        .expect("recall");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].record.id, memory_id);
    assert!(hits[0].score > 0.99);

    let related = MemoryRecord::new(
        seed.id,
        "Vector state is derived and repairable from durable Connectome truth.",
        vec![0.8, 0.2, 0.0, 0.0],
        MemoryKind::Fact,
    );
    let related_id = store
        .save_memory(&scope, related)
        .await
        .expect("save related memory");

    let edge = ReasonEdge::new(memory_id, related_id, "supports");
    let edge_id = store.relate(&scope, edge).await.expect("relate");
    let edges = store
        .edges_for(&scope, memory_id)
        .await
        .expect("edges for memory");
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].id, edge_id);
    assert_eq!(edges[0].relation, "supports");

    let archive = ArchiveBlob::new(memory_id, seed.id, b"archived-rro-context".to_vec(), "raw-test");
    store
        .write_archive(&scope, archive)
        .await
        .expect("write archive");
    let archive_bytes = store
        .read_archive_bytes(&scope, memory_id)
        .await
        .expect("read archive")
        .expect("archive exists");
    assert_eq!(archive_bytes, b"archived-rro-context".to_vec());

    let vector_id = VectorId::new();
    store
        .record_alignment(AlignmentJournalEntry::new(
            seed.id,
            Some(memory_id),
            Some(vector_id),
            VectorAlignmentStatus::Vectorized,
            1,
        ))
        .await
        .expect("record alignment");

    let alignments = store
        .alignment_for_rro(seed.id)
        .await
        .expect("alignment for rro");
    assert_eq!(alignments.len(), 1);
    assert_eq!(alignments[0].source_rro, seed.id);
    assert_eq!(alignments[0].memory_id, Some(memory_id));
    assert_eq!(alignments[0].vector_id, Some(vector_id));
    assert_eq!(alignments[0].status, VectorAlignmentStatus::Vectorized);
}

#[tokio::test]
async fn memory_adapter_enforces_project_scope() {
    let store = InMemoryBrainStore::default();
    let initialized_scope = test_scope();
    let other_scope = ProjectScope::new(
        initialized_scope.workspace_id,
        ProjectSlug::new("other-project").expect("valid slug"),
    );
    store
        .init_project(&initialized_scope)
        .await
        .expect("project init");

    let seed = ReasonReadyObject::new(initialized_scope.workspace_id, RroKind::SeedChat, "seed");
    let memory = MemoryRecord::new(seed.id, "scoped memory", vec![1.0], MemoryKind::Project);

    let err = store
        .save_memory(&other_scope, memory)
        .await
        .expect_err("uninitialized scope should fail");
    assert!(err.to_string().contains("project scope other-project"));
}
