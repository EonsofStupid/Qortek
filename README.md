# Qortek

Qortek is the public Reason Ready Daemon substrate for building AI systems that need durable context, graph-backed memory, vector recall alignment, forked reasoning workflows, and mesh-aware knowledge estate nodes.

This repository is the public, clean, maintainable surface. It is designed to develop alongside the private `qortex` vector engine work without exposing private release code before it is ready.

## What Qortek is

Qortek is not just a vector database wrapper. It is the durable brain layer that keeps the operator's context coherent:

- Reason Ready Objects (RROs)
- persistent memory declarations
- graph relationships and lineage
- fork / deliberate / commit / foldback workflows
- audit and event history
- vector recall alignment journals
- MCP / AI-mesh node discovery contracts
- storage and vector adapter traits

The vector engine and the durable brain are separate by design:

```text
Qortek Brain Store  = truth, persistence, graph, lineage, policy, events
Qortex / Qdrant     = vector recall, hybrid search, ANN, sparse/dense retrieval
RRD                 = orchestration layer that keeps both aligned
```

## Current status

`0.1.0-alpha.0` is the public scaffold. The initial repo establishes the contracts, crate layout, docs, examples, and CI foundation. It does not claim production readiness.

## Public/private boundary

- `Qortek` is public and contains stable contracts, docs, examples, and adapter interfaces.
- `qortex` remains private while the embedded vector engine and release path are finalized.
- Qortek must remain usable without private qortex code.
- Public adapters may support upstream engines first, with private accelerated adapters layered later.

## Workspace layout

```text
crates/
  qortek-core       IDs, errors, shared primitives
  qortek-rro        Reason Ready Object model
  qortek-store      durable brain-store traits
  qortek-vector     vector recall traits
  qortek-fork       fork/deliberate/commit/foldback model
  qortek-mesh       AI node transport contracts
  qortek-preflight  resource and route planning
  qortek-rrd        daemon-facing orchestration shell

adapters/
  qortek-adapter-memory   in-memory development adapter
  qortek-adapter-qdrant   public Qdrant-facing placeholder
  qortek-adapter-surreal  Surreal-compatible placeholder

examples/
  basic-rro
  fork-foldback
```

## Core pattern

```text
Prompt starts
→ seed RRO created
→ preflight resolves workspace/resources/mesh
→ brain store persists state
→ vector store indexes recall artifacts
→ ambiguity can create a fork tab
→ fork reaches commit condition
→ foldback collapses branch result into seed context
→ memory candidate becomes durable
```

## Build

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Roadmap

See [`docs/ROADMAP.md`](docs/ROADMAP.md).

## License

Apache-2.0. See [`LICENSE`](LICENSE) and [`NOTICE.md`](NOTICE.md).
