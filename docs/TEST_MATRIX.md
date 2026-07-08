# Test Matrix

This matrix defines the tests Qortek must grow into. A test listed here is not complete until it exists in code, runs in CI or documented local verification, and has a replay or evidence trail.

## Legend

| Status | Meaning |
|---|---|
| planned | Test not written yet. |
| scaffolded | Test file exists but coverage is partial. |
| implemented | Test runs. |
| verified | Test runs and evidence is recorded. |

## Core contract tests

| Area | Test | Status |
|---|---|---|
| RRO | create seed RRO | planned |
| RRO | serialize/deserialize RRO | planned |
| RRO | lifecycle transition validation | planned |
| BrainStore | put/get RRO | planned |
| BrainStore | save memory | planned |
| BrainStore | relate records | planned |
| BrainStore | write/read archive bytes | planned |
| BrainStore | project slug isolation | planned |
| VectorRecall | upsert vector batch | planned |
| VectorRecall | dense search | planned |
| VectorRecall | hybrid search | planned |
| Fork | open fork | planned |
| Fork | commit fork | planned |
| Fork | foldback record | planned |
| Mesh | advertise node | planned |
| Mesh | discover by capability | planned |
| Preflight | local embedded plan | planned |
| Preflight | remote/fabric plan | planned |

## Adapter tests

| Adapter | Test | Status |
|---|---|---|
| memory | BrainStore contract suite | planned |
| memory | archive round-trip | planned |
| memory | project isolation | planned |
| surreal | BrainStore contract suite | planned |
| surreal | scoped namespace behavior | planned |
| surreal | archive round-trip | planned |
| qdrant | VectorRecall contract suite | planned |
| qortex | VectorRecall contract suite | private-track |

## Alignment tests

| Test | Status |
|---|---|
| create alignment journal entry | planned |
| mark vector pending | planned |
| mark vectorized | planned |
| detect stale vector | planned |
| repair stale vector | planned |
| rebuild vector state from BrainStore | planned |

## Replay tests

| Replay | Required proof | Status |
|---|---|---|
| basic RRO | example runs | scaffolded |
| fork foldback | example runs | scaffolded |
| memory recall | save + recall + relate | planned |
| vector alignment | brain write + vector write + repair | planned |
| mesh preflight | discover + capability route | planned |

## Benchmark tests

These are planned for later stages. Do not claim performance wins until these exist.

| Benchmark | Status |
|---|---|
| HTTP Qdrant baseline | private-track |
| qortex embedded | private-track |
| qortex dedicated recall service | private-track |
| memory adapter p50/p95 | planned |
| surreal adapter p50/p95 | planned |
| graph + vector fusion quality | planned |
