# Verification

Verification is the difference between a working foundation and a convincing story.

## Status language

Use these words consistently:

| Word | Meaning |
|---|---|
| planned | No implementation exists yet. |
| scaffolded | Files/API shape exist; behavior is incomplete. |
| experimental | Behavior exists but may change. |
| implemented | Code exists and tests pass. |
| verified | Code/tests/docs/replay evidence all exist. |
| private-track | Implementation exists or is planned outside this public repo. |

Do not use `done`, `turnkey`, `production-ready`, or `drop-in` unless the matching gate below is satisfied.

## Required commands

Public workspace verification:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Crate-local verification:

```bash
cargo check -p <crate>
cargo test -p <crate>
```

Example verification:

```bash
cargo run -p basic-rro
cargo run -p fork-foldback
```

## Documentation verification

For docs-only changes:

- [ ] Relative links are valid by inspection.
- [ ] No private source, secrets, or private endpoints appear.
- [ ] Status language is accurate.
- [ ] Roadmap remains consistent.
- [ ] Session replay updated if the doc changes the execution flow.

## Contract verification

For trait/API changes:

- [ ] Public types derive or implement expected serialization traits when needed.
- [ ] Existing examples still compile.
- [ ] Memory adapter compiles against the new contract.
- [ ] No private adapter is required for public tests.
- [ ] Docs show the contract boundary.

## Adapter verification

For adapters:

- [ ] Adapter implements the public trait without leaking backend internals.
- [ ] Adapter has contract tests shared with other adapters.
- [ ] Adapter has a documented lifecycle.
- [ ] Adapter has a documented failure mode.
- [ ] Adapter is feature-gated if it requires an optional dependency.

## Alignment verification

For any change touching BrainStore and VectorRecall:

- [ ] A durable object ID maps to vector ID(s).
- [ ] Alignment status is written.
- [ ] Stale vector state can be detected.
- [ ] Repair can be replayed from BrainStore truth.
- [ ] Vector state is never treated as sole truth.

## Replay verification

A session replay must include:

- goal;
- input artifacts;
- commands run;
- expected outputs;
- actual outputs or explicit unverified status;
- files changed;
- next action.

## Release verification

Before tagging a release:

- [ ] `docs/RELEASE_CHECKLIST.md` complete.
- [ ] `docs/ROADMAP.md` matches current state.
- [ ] Public/private boundary reviewed.
- [ ] CI green or failure documented.
- [ ] At least one session replay proves the intended first-run path.
