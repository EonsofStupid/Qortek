# Protocol Strategy

Qortek should not invent a protocol before proving where existing standards fail.

The public strategy is layered adoption:

```text
MCP = tools, data, and context access
A2A / Agent2Agent = agent-to-agent discovery, delegation, and task exchange
ANP / ACP = research targets for wider decentralized or multimodal agent communication
Qortek Mesh = local policy, preflight, capability scoping, and estate-node routing
```

## MCP role

MCP is the tool/data/context access layer. A Clyffy estate can expose tools, context stores, retrieval services, or project resources through MCP-style servers.

## A2A role

A2A is the best public anchor for agent-to-agent communication. Qortek should treat it as the likely external interoperability layer for:

- agent discovery
- capability cards
- task delegation
- cross-estate handoff
- external agent compatibility

## Qortek Mesh role

Qortek Mesh is not a replacement for MCP or A2A. It is the internal control and routing layer that decides:

- which estate node is trusted;
- which capability is allowed;
- whether local embedded mode is enough;
- whether fabric-adjacent routing is available;
- whether HTTP/QUIC fallback should activate;
- which context must be prepared before sending anything out.

## Tin-can handshake

The informal model is:

```text
Clyffy out in the world
  -> preflight asks whether a known estate is present
  -> estate node answers with capabilities and policy
  -> Qortek decides whether ingestion/retrieval is allowed
  -> RRD prepares context and opens the recall path
```

This must be explicit, audited, and denied by default.

## Performance rule

Qortek should preserve the target of outperforming HTTP-only vector retrieval by keeping local/fabric-adjacent recall paths available and using HTTP/QUIC only when the route requires it.
