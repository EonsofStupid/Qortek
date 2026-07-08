# Turnkey Path

Qortek should be easy to try without requiring private infrastructure.

Turnkey does not mean production-ready. Turnkey means a developer can clone the repo, run the examples, understand the model, and see why the architecture matters.

## Audience

Qortek is for builders experimenting with AI memory moats, durable reasoning stores, vector recall, graph relationships, and agent/estate routing.

## Why try it

Try Qortek if you need:

- durable AI memory instead of ephemeral chat history;
- a public contract between graph memory and vector recall;
- fork/deliberate/commit/foldback workflows;
- replayable reasoning artifacts;
- backend-agnostic BrainStore adapters;
- VectorRecall adapters that can target Qdrant, qortex, or another engine;
- a clean way to test local-first and estate-node AI infrastructure.

Do not try Qortek yet if you need:

- a production database;
- a finished qortex engine;
- a managed hosted service;
- a full MCP/A2A implementation;
- guaranteed benchmark wins.

## First-run path

```bash
git clone https://github.com/EonsofStupid/Qortek.git
cd Qortek
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p basic-rro
cargo run -p fork-foldback
```

## Expected result

- The workspace compiles.
- Examples run without private services.
- The user sees an RRO shape and a fork/foldback shape.
- No qortex private repo is required.
- No ClyffyOS service is required.

## Turnkey maturity levels

### Level 0 — scaffolded

- Docs exist.
- Crates exist.
- Memory adapter exists.
- Examples compile.

### Level 1 — local usable

- BrainStore contract tests pass.
- Memory adapter supports save/recall/relate/archive behavior.
- Session replay examples are executable.

### Level 2 — adapter usable

- Surreal-compatible adapter passes the same contract tests.
- Qdrant adapter passes vector recall tests.
- Alignment journal can repair stale vector state.

### Level 3 — distributed-ready

- Mesh preflight can resolve a node.
- Capability-gated calls are enforced.
- Dedicated recall service can run separately.
- Replays prove local and remote modes.

### Level 4 — release candidate

- CI green.
- Benchmark evidence exists.
- Docs match implementation.
- No private dependency required for public usage.

## Turnkey rule

A feature is not turnkey until a new user can run it using only public repo instructions.
