# Rollout Plan

Qortek rollout is staged. Each stage must preserve public usability and avoid private dependency lock-in.

## Stage 0 — public scaffold

Goal: cloneable public workspace with honest docs and no private dependency.

Required:

- [x] Rust workspace exists.
- [x] README exists.
- [x] public/private boundary exists.
- [x] AGENTS execution contract exists.
- [x] preflight and verification docs exist.
- [x] memory adapter scaffold exists.
- [x] examples exist.

Exit:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Stage 1 — public contract hardening

Goal: make Qortek's public contracts stable enough for adapters.

Required:

- [ ] Project slug / namespace primitive.
- [ ] MemoryRecord.
- [ ] RecallHit.
- [ ] ReasonEdge.
- [ ] ArchiveBlob.
- [ ] AlignmentJournalEntry.
- [ ] BrainStore contract test suite.
- [ ] VectorRecall contract test suite.

Exit:

```bash
cargo test -p qortek-store
cargo test -p qortek-vector
cargo test -p qortek-adapter-memory
```

## Stage 2 — replayable local moat

Goal: prove a local-only memory moat without private qortex or ClyffyOS.

Required:

- [ ] Save memory.
- [ ] Recall memory.
- [ ] Relate memory.
- [ ] Write archive bytes.
- [ ] Read archive bytes.
- [ ] Session replay proving the flow.

Exit:

```bash
cargo run -p replay-memory-moat
```

## Stage 3 — adapter parity

Goal: prove the public contracts can support multiple backends.

Required:

- [ ] Memory adapter passes contract tests.
- [ ] Surreal-compatible adapter passes contract tests.
- [ ] Qdrant adapter passes VectorRecall contract tests.
- [ ] Backend-specific behavior stays behind adapters.

Exit:

```bash
cargo test --workspace --features surreal,qdrant
```

## Stage 4 — alignment journal

Goal: prove the durable brain can rebuild recall state.

Required:

- [ ] Brain write emits alignment journal state.
- [ ] Vector write links to durable source.
- [ ] Stale vectors detected.
- [ ] Repair path implemented.
- [ ] Replay proves rebuild from brain truth.

Exit:

```bash
cargo test -p qortek-store alignment
cargo run -p replay-vector-alignment
```

## Stage 5 — qortex private bridge

Goal: let private qortex prove high-performance recall through public Qortek traits without making Qortek depend on it.

Required:

- [ ] qortex facade crate exists in private repo.
- [ ] Qortek qortex adapter compiles only when private feature/path is supplied.
- [ ] Public repo still compiles without qortex.
- [ ] Benchmark compares qortex against HTTP Qdrant.

Exit:

```bash
cargo test --workspace
# Private-track only:
cargo test -p qortek-adapter-qortex --features private-qortex
```

## Stage 6 — ClyffyOS service integration

Goal: ClyffyOS serves Qortek + Qortex + Surreal brain over a capability-computed wire.

Required:

- [ ] clyffyos-wire owns DTOs.
- [ ] clyffyd serves `/v1/capability`.
- [ ] clyffyd exposes recall and memory verbs.
- [ ] DevPulse connects as client.
- [ ] Offline DevPulse still works.

Exit:

```text
DevPulse disconnected = Tier-1 available.
DevPulse connected = Tier-2 capability computed from live ClyffyOS.
No fake stats.
```

## Stage 7 — public alpha

Goal: public users can understand, run, and extend Qortek.

Required:

- [ ] All Stage 1-4 public gates green.
- [ ] README matches implementation.
- [ ] Test matrix statuses updated.
- [ ] At least three session replays exist.
- [ ] Release checklist complete.

Exit:

```bash
git tag v0.1.0-alpha.0
git push origin v0.1.0-alpha.0
```
