# Preflight

Preflight prevents Qortek from becoming a pile of hopeful code. Run it before implementation, before release, and before any AI agent claims a task is complete.

## 1. Boundary preflight

- [ ] Is this change public-safe?
- [ ] Does it avoid private qortex source?
- [ ] Does it avoid private ClyffyOS source?
- [ ] Does it avoid private legal/waiver text?
- [ ] Does it avoid secrets, tokens, hostnames, and local-only paths?
- [ ] Does it keep Qortek useful without private dependencies?

Stop if any answer is no.

## 2. Architecture preflight

- [ ] Does BrainStore remain durable truth?
- [ ] Does VectorRecall remain rebuildable recall infrastructure?
- [ ] Is alignment state explicit when both stores are touched?
- [ ] Are backend-specific details kept inside adapters?
- [ ] Does the change preserve project/workspace scoping?
- [ ] Does the change preserve the public/private boundary?

## 3. Implementation preflight

Before editing, identify:

```text
goal:
files:
crate(s):
feature flags:
new dependencies:
verification command:
done criteria:
rollback path:
```

## 4. Dependency preflight

For each new dependency:

- [ ] License is compatible with Apache-2.0 public distribution.
- [ ] Dependency is necessary for the public repo.
- [ ] Dependency does not pull private code.
- [ ] Dependency does not make examples require external services by default.
- [ ] Dependency is not added just to satisfy a future private integration.

## 5. Runtime preflight

If the change affects runtime behavior:

- [ ] Is it embedded, local service, remote service, or adapter-only?
- [ ] What owns lifecycle?
- [ ] What owns shutdown?
- [ ] What is the health check?
- [ ] What is the failure mode?
- [ ] What is the recovery path?

## 6. Verification preflight

Pick the narrowest valid command first, then broader checks:

```bash
cargo check -p <crate>
cargo test -p <crate>
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Docs-only work must still be reviewed for boundary, link accuracy, and status language.

## 7. Done preflight

Before saying done:

- [ ] Code/docs committed.
- [ ] Tests or docs checks completed, or limitation stated.
- [ ] Session replay updated if the work changes architecture/workflow.
- [ ] Roadmap updated if the work changes status.
- [ ] Any incomplete item is clearly marked planned or scaffolded.
