# Mesh

Qortek's mesh layer describes identity-bound, capability-scoped AI node communication.

It is not a VPN implementation. It is the contract layer that allows an implementation to choose local embedded, fabric-adjacent, HTTP, or QUIC transport profiles.

## Transport profiles

- `LocalEmbedded`
- `FabricAdjacent`
- `RemoteHttp`
- `RemoteQuic`

## Intended use

Preflight chooses a route based on local resources, available estate nodes, trust, and policy.

```text
local embedded brain/vector
  -> fabric-adjacent estate node
  -> remote HTTP/QUIC node
```

## Security direction

Mesh traffic should eventually be:

- capability-scoped
- identity-bound
- auditable
- workspace-aware
- denied by default
