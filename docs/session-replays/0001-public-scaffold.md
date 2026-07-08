# Replay: public scaffold

- Date: 2026-07-08
- Status: scaffolded
- Scope: public Qortek repository setup
- Repos touched: `EonsofStupid/Qortek`
- Public/private boundary risk: low; docs and public contracts only

## Goal

Create a public repository foundation for Qortek that can develop alongside private qortex work without exposing private source.

## Starting state

The repository existed and was public but empty.

## Steps

1. Added README with public/private boundary.
2. Added Rust workspace manifest.
3. Added core crates for IDs, RROs, BrainStore, VectorRecall, fork lifecycle, mesh, preflight, and RRD shell.
4. Added memory adapter and placeholder qdrant/surreal adapter crates.
5. Added examples for basic RRO and fork/foldback.
6. Added CI, license, contribution guide, security policy, issue templates, and PR template.
7. Added architecture docs.
8. Added AGENTS execution contract, preflight, verification, test matrix, rollout, and replay structure.

## Commands run

No local commands were successfully run from the assistant environment because the sandbox could not resolve `github.com` for cloning at the time of setup.

Expected user/local verification:

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

- Workspace compiles.
- Examples run.
- Public docs explain why Qortek exists.
- No private qortex or ClyffyOS source required.

## Actual result

Repository files were committed through the GitHub connector. Local compile was not verified from the assistant sandbox.

## Files changed

Representative files:

- `README.md`
- `AGENTS.md`
- `Cargo.toml`
- `crates/qortek-*`
- `adapters/qortek-adapter-*`
- `examples/basic-rro`
- `examples/fork-foldback`
- `docs/*`
- `.github/*`

## Verification evidence

Committed to GitHub. CI status must be checked separately.

## Not verified

- Local compile.
- CI pass.
- Example runtime output.

## Next action

Run the verification commands locally or in CI, fix compile issues, and update this replay with actual command output.
