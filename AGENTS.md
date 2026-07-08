# AGENTS.md — Qortek AI Execution Contract

This file is the first document an AI coding agent must read before changing Qortek.

Qortek is a public, safe, adapter-first Reason Ready Daemon substrate. It must remain useful without private qortex code, private ClyffyOS code, or operator-specific secrets.

## Prime directive

Do not present work as done until the relevant verification commands, tests, docs, and evidence are complete. If verification cannot run in the current environment, state exactly what was not verified.

## Repository boundary

Allowed here:

- public contracts;
- public Rust crates;
- docs and diagrams in text form;
- examples that do not require private repos;
- memory/vector/store/mesh traits;
- adapter placeholders and public adapters;
- session replays that contain no secrets.

Not allowed here:

- private qortex source;
- private ClyffyOS source;
- private waiver/legal text;
- secrets, tokens, credentials, local hostnames, private endpoints;
- private operator notes unless rewritten into public-safe architecture;
- claims that require benchmarks unless benchmark evidence exists.

## Required work loop

Every meaningful change must follow this loop:

```text
1. Preflight
2. Plan
3. Edit
4. Verify
5. Record evidence
6. Update session replay
7. Report only what is proven
```

## Preflight checklist

Before editing:

- [ ] Read `README.md`.
- [ ] Read this `AGENTS.md`.
- [ ] Read `docs/PREFLIGHT.md`.
- [ ] Read `docs/PUBLIC_BOUNDARY.md`.
- [ ] Identify whether the change is public contract, adapter, docs, test, or example.
- [ ] Confirm no private source is required.
- [ ] Confirm no private dependency is introduced.
- [ ] Confirm the expected verification command.

Stop immediately if the task requires private code that was not explicitly approved.

## Planning requirements

Before implementation, write down:

- goal;
- files expected to change;
- public/private boundary risk;
- verification command;
- done criteria;
- rollback path.

For small changes, this may live in the PR summary. For multi-step work, update or create a session replay under `docs/session-replays/`.

## Verification hierarchy

Use the narrowest command that proves the change, then the broader commands before release.

Minimum local verification:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

If a change is docs-only, at minimum verify:

- links are relative and valid by inspection;
- no private terms or secrets are present;
- public/private boundary remains accurate.

## Done criteria

A task is done only when:

- code or docs are committed;
- verification evidence is recorded;
- session replay is updated when the task changes architecture or workflow;
- README or roadmap is updated if the user-facing path changed;
- unfinished pieces are explicitly marked as planned, not implied complete.

## Language rules

Use precise status words:

- `implemented` means code exists and tests passed;
- `scaffolded` means shape exists but behavior is incomplete;
- `planned` means no implementation yet;
- `experimental` means implementation exists but the API or behavior may change;
- `private-track` means the public repo intentionally does not contain the implementation.

Never use `done`, `production-ready`, `turnkey`, or `drop-in` unless the verification gates in `docs/VERIFICATION.md` are satisfied.

## Architecture invariant

```text
BrainStore owns durable truth.
VectorRecall owns rebuildable recall indexes.
The alignment journal proves and repairs drift.
Mesh/preflight decides where work may run.
Adapters keep backends replaceable.
```

## Commit hygiene

Every commit should be small enough to explain in one sentence. Prefer:

```text
docs: add preflight checklist
core: add project scope primitive
adapter: scaffold surreal brain store
example: add basic memory replay
```

Avoid mixed commits that combine architecture, code, tests, and unrelated cleanup.
