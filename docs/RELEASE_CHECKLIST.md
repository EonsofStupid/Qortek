# Release Checklist

Use this before tagging any public release.

## Code

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] examples compile and run

## Boundary

- [ ] no private qortex source
- [ ] no private waiver/legal text
- [ ] no secrets
- [ ] no local absolute paths
- [ ] no production claims without tests or evidence

## Docs

- [ ] README status matches implementation
- [ ] ROADMAP updated
- [ ] adapter status is honest
- [ ] public/private boundary reviewed

## Tagging

Suggested first tag after CI is green:

```bash
git tag v0.1.0-alpha.0
git push origin v0.1.0-alpha.0
```
