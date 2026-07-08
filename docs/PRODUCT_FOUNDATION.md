# Product Foundation — Qortek, Qortex, Connectome, Edge Estates

This document captures the actual product foundation before any turnkey packaging claims.

The goal is not a generic vector database wrapper. The goal is a local-first, estate-aware AI memory and reasoning substrate where Qortex, Connectome, and edge estates work together in a boring, predictable, repeatable pattern.

## Core thesis

```text
Qortex = embedded/dedicated recall heart
Connectome = durable AI brain and relationship layer
Qortek = public contracts and orchestration substrate
Edge Estate = standardized place where Clyffy can warp, work, cache, remember, and sync
ClyffyOS = headless runtime that runs this on serious local/GB10 infrastructure
DevPulse = operator cockpit/client
```

Qortek must be built from the working foundation upward:

1. prove local BrainStore behavior;
2. prove Qortex embedded recall behavior;
3. prove Connectome relationships and durable memory;
4. prove BrainStore and Qortex alignment;
5. prove estate-local MCP/Clyffy endpoint bootstrap;
6. prove edge discovery and preflight;
7. only then package it as turnkey.

## Non-negotiable product rules

- Brain truth is durable.
- Vector recall is fast but rebuildable.
- Estate-local context stays local to the estate unless policy allows export.
- Every estate uses the same boring structure.
- Every estate may differ in content, but not in shape.
- Every operation must be replayable.
- Every claim of completion must have verification evidence.
- Public Qortek must not require private qortex code to build.
- Private qortex must prove performance claims against HTTP Qdrant before those claims are public.

---

# 1. Qortex — recall heart

Qortex is the vector recall engine. It begins from the Qdrant-derived fork, but the product target is an owned embedded/dedicated recall engine.

## Required product shape

Qortex must support three deployment modes:

```text
embedded in-process
local dedicated recall service
remote/fabric/edge recall service
```

## Required API surface

Qortex must eventually expose a stable facade:

```text
open
close
create_collection
delete_collection
upsert
retrieve
search_dense
search_sparse
search_hybrid
scroll
count
flush
snapshot
health
```

## Required guarantees

- Preserve Qdrant-compatible capability until parity is proven.
- Do not rip out HTTP/server paths before embedded and dedicated recall are benchmarked.
- Keep `lib/edge` as internal machinery until the Qortex facade is real.
- Qortek adapters must call Qortex through the facade, not through Qdrant internals.
- Qortex must benchmark against HTTP Qdrant with p50/p95/p99, cold/warm, and rebuild/recovery data.

## First proof

Qortex is real when this works:

```text
create local collection
upsert memory vectors
search dense
search hybrid
return source memory ids
flush
restart
search again
```

---

# 2. Connectome — custom durable brain inspired by SurrealDB

Connectome is the durable intelligence layer. It is the custom brain that takes the parts wanted from SurrealDB-style thinking: document + graph + vector refs + realtime/event relationships + temporal facts.

Connectome is not just a database. It is the relationship substrate that lets Clyffy understand what a vector hit means.

## Connectome owns

- estate identity;
- project/workspace scope;
- memory records;
- RROs;
- graph relationships;
- fork/foldback lineage;
- tool traces;
- evidence references;
- archive blobs;
- policy boundaries;
- event journal;
- alignment journal;
- last-touched state;
- estate-local history;
- repair/replay state.

## Connectome must answer

- What is this memory?
- Where did it come from?
- What estate owns it?
- What project/domain/subdomain does it belong to?
- What vector refs point to it?
- What graph edges surround it?
- What fork created it?
- What evidence supports it?
- Is it current, stale, contradicted, or archived?
- Can this Clyffy instance access it?

## Connectome and Qortex harmony

```text
Connectome writes durable memory
  -> alignment journal marks PendingVectorization
  -> embedder creates vector
  -> Qortex indexes vector
  -> alignment journal marks Vectorized
  -> recall returns vector hit
  -> Connectome resolves hit into memory + graph + estate context
```

If Qortex drifts, Connectome wins.

```text
Brain truth wins.
Vector state is rebuilt.
Journal proves what happened.
```

---

# 3. Edge Estate — standardized warpable domain

An estate is a place where Clyffy can arrive, discover the local context, work safely, and leave the cache/history organized for the next visit.

An estate can be:

- a machine;
- a repo;
- a project folder;
- a business/customer domain;
- a GB10 node;
- a local network service;
- a remote knowledge estate;
- a future ClyffyOS site.

## Estate principle

Every estate must be similar in shape but different in content.

The operator should be able to recognize the same structure everywhere:

```text
.clyffy/
  estate.toml
  endpoint.toml
  mcp/
  connectome/
  qortex/
  cache/
  history/
  sessions/
  replays/
  policies/
  tools/
  artifacts/
  telemetry/
```

## Estate-local state

The estate keeps:

- cache;
- local history;
- last touched files/domains;
- local memory candidates;
- local graph edges;
- session replays;
- tool traces;
- ingestion state;
- sync state;
- policy decisions.

This avoids dumping everything into one global context pile.

## Last-touched index

Every estate must expose a last-touched ledger:

```text
last_touched:
  actor: clyffy | operator | tool | remote-agent
  domain: memory | recall | files | tools | policy | replay | graph
  path: <estate-relative path>
  operation: read | write | ingest | recall | repair | sync
  timestamp: <utc>
  replay_id: <optional>
```

This lets Clyffy return later and immediately know what changed.

---

# 4. Estate bootstrap — MCP folder/files + Clyffy endpoint

When a new estate is established, Qortek should support deploying a small predictable estate kit.

## Estate kit

```text
.clyffy/estate.toml        estate identity and metadata
.clyffy/endpoint.toml      local Clyffy endpoint descriptor
.clyffy/mcp/               MCP tool/context definitions
.clyffy/policies/          estate access rules
.clyffy/replays/           replay log folder
.clyffy/connectome/        local relationship state or pointers
.clyffy/qortex/            local recall state or pointers
.clyffy/cache/             local cache
.clyffy/history/           local history
```

## Endpoint descriptor

The endpoint descriptor should include:

```text
estate_id
estate_name
endpoint_url
transport_profile
capabilities
policy_profile
last_seen
last_touched
connectome_version
qortex_version
mcp_manifest_path
```

## MCP role

MCP is the estate's local tool/context surface.

It should describe:

- available tools;
- allowed file roots;
- local context sources;
- ingestion rules;
- project-specific commands;
- restricted paths;
- evidence sources;
- sync targets.

## Clyffy warp behavior

When Clyffy reaches an estate:

```text
1. find estate descriptor
2. validate endpoint descriptor
3. load MCP manifest
4. run policy preflight
5. read last-touched ledger
6. inspect cache/history freshness
7. query Connectome for local relationships
8. query Qortex for recall if available
9. begin work with estate-local context
10. append replay and last-touched records
```

---

# 5. Edge networking

Edge networking is not just remote HTTP.

It is the ability for Clyffy to discover or attach to a local estate, determine what it is allowed to do, and route work through the right local/fabric/remote path.

## Transport profiles

```text
LocalEmbedded       same process or local library
LocalService        local daemon/service
FabricAdjacent      GB10/fabric-adjacent node
LanHttp             LAN HTTP endpoint
LanQuic             LAN QUIC endpoint
MeshAddress         Tailscale/Netbird/WireGuard-style address
RemoteHttp          normal remote endpoint
RemoteQuic          future remote optimized path
```

## Preflight sequence

```text
discover estate
load estate.toml
load endpoint.toml
check trust
check policy
check capability
check transport
check freshness
check local cache/history
check Connectome status
check Qortex status
choose route
start replay
execute
record result
```

## Hard rule

Clyffy must not assume an estate is safe just because it exists.

Discovery is not trust. Capability is not permission. Permission is not execution. Execution must be replayed.

---

# 6. Standard estate domains

Each estate should organize work into predictable domains.

## Required domains

```text
identity       estate id, owner, purpose, trust profile
policy         access rules, allowed roots, denied paths
memory         durable memories and declarations
recall         vector collections, embedding refs, qortex state
connectome     graph relationships and RRO lineage
ingestion      file/doc/repo ingestion state
cache          short-lived computed context
history        durable local history summaries
sessions       active and previous Clyffy sessions
replays        execution records and verification notes
tools          MCP tools and allowed commands
artifacts      generated outputs, plans, reports
telemetry      metrics, health, last touched, repair stats
sync           remote sync state and conflict records
```

## Domain rule

Each domain must have:

- manifest;
- last touched entry;
- owner;
- retention policy;
- verification method;
- repair behavior if applicable.

---

# 7. Methodical build order

Do not package before the foundation works.

## Stage A — local public proof

Build in Qortek:

- project scope;
- memory record;
- BrainStore save/get;
- recall-by-embedding;
- relation edges;
- archive write/read;
- alignment journal;
- replay example.

Proof:

```text
local memory saved
local memory recalled
edge created
archive read back
alignment state recorded
replay file written
```

## Stage B — Qortex embedded proof

Build in qortex:

- facade crate;
- embedded open;
- collection create;
- upsert;
- dense search;
- hybrid search;
- flush/restart;
- benchmark vs HTTP Qdrant.

Proof:

```text
Qortex returns stable recall after restart
Qortex result maps back to memory id
HTTP Qdrant baseline exists
```

## Stage C — Connectome + Qortex harmony

Build integration:

- Connectome writes memory;
- alignment journal marks pending;
- embedder creates vector;
- Qortex indexes;
- recall returns vector hit;
- Connectome resolves hit into graph context;
- stale vector repair works.

Proof:

```text
BrainStore can rebuild Qortex index from durable truth
```

## Stage D — estate kit proof

Build estate bootstrap:

- `.clyffy/estate.toml`;
- `.clyffy/endpoint.toml`;
- `.clyffy/mcp/manifest.toml`;
- `.clyffy/replays/`;
- `.clyffy/history/last_touched.jsonl`;
- preflight command.

Proof:

```text
new estate initialized
preflight reads descriptor
MCP manifest discovered
last-touched ledger updated
session replay written
```

## Stage E — edge routing proof

Build edge preflight:

- local estate discovery;
- endpoint capability check;
- trust/policy check;
- route choice;
- cache/history check;
- replay start/finish.

Proof:

```text
Clyffy can arrive at estate, inspect state, decide route, execute allowed operation, and leave replay evidence
```

## Stage F — turnkey packaging

Only after stages A-E:

- docs;
- installer/setup;
- examples;
- templates;
- replay demos;
- public alpha release.

---

# 8. Verification before done

No stage is complete until it has:

- code;
- tests;
- example or replay;
- docs updated;
- verification command output;
- clear status language.

Accepted status words:

```text
planned
scaffolded
implemented
verified
private-track
```

Forbidden without evidence:

```text
done
turnkey
production-ready
drop-in
faster than HTTP Qdrant
```

---

# 9. Product foundation checklist

## Qortek public core

- [ ] ProjectScope / ProjectSlug
- [ ] MemoryRecord
- [ ] ReasonEdge
- [ ] ArchiveBlob
- [ ] AlignmentJournalEntry
- [ ] BrainStore trait
- [ ] VectorRecall trait
- [ ] memory adapter
- [ ] local replay example
- [ ] contract tests

## Qortex private engine

- [ ] owned downstream policy
- [ ] parity matrix
- [ ] facade crate
- [ ] embedded collection lifecycle
- [ ] dense search
- [ ] hybrid search
- [ ] flush/restart
- [ ] dedicated recall service
- [ ] HTTP Qdrant benchmark

## Connectome

- [ ] graph model
- [ ] temporal fact model
- [ ] RRO lineage
- [ ] fork/foldback lineage
- [ ] memory declaration model
- [ ] archive model
- [ ] alignment repair loop
- [ ] estate-local history

## Estate edge

- [ ] estate descriptor
- [ ] endpoint descriptor
- [ ] MCP manifest
- [ ] policy profile
- [ ] last-touched ledger
- [ ] cache/history structure
- [ ] replay structure
- [ ] preflight command
- [ ] route choice

## Clyffy workflow

- [ ] discover estate
- [ ] check trust
- [ ] load MCP
- [ ] inspect last touched
- [ ] attach Connectome
- [ ] attach Qortex
- [ ] execute allowed work
- [ ] write replay
- [ ] update estate history

---

# 10. Final foundation statement

Qortek succeeds only if a new estate can be initialized, recognized, worked inside, and revisited later with its memory, cache, graph, history, and recall pathways intact.

The product is not the docs.

The product is the repeatable loop:

```text
establish estate
preflight estate
load MCP/context
write durable brain state
index recall through Qortex
resolve recall through Connectome
execute work
record replay
update last touched
return later with continuity
```
