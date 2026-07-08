# Contributing

Qortek is currently in alpha scaffold stage. Contributions should preserve the public/private boundary:

- public contracts and examples belong here;
- private qortex release work does not belong here until intentionally published;
- adapters must be modular and feature-gated;
- claims must match implemented code.

## Local checks

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Design rules

1. Brain-store truth is durable and rebuilds vector state.
2. Vector recall is aligned through explicit IDs and journal states.
3. Async is used at the edge; long-lived stores own their workers.
4. Public APIs should favor stable traits over leaking backend internals.
5. Docs must identify whether a feature is implemented, planned, or private-roadmap.
