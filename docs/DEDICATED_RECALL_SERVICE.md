# Dedicated Recall Service

Qortek supports an embedded-first model, but the long-term architecture should allow recall to move behind a dedicated service boundary when distribution or performance requires it.

## Why

HTTP-only vector retrieval can be too slow and too generic for local AI infrastructure. Qortek should preserve faster paths:

- in-process embedded recall
- local IPC recall
- fabric-adjacent recall
- dedicated recall service
- HTTP/QUIC fallback

## Service boundary

A dedicated recall service should own:

- Qortex vector engine handle
- collection lifecycle
- vector upserts
- sparse/dense/hybrid search
- rerank candidate packaging
- repair/reindex jobs
- recall telemetry

The BrainStore remains durable truth. The recall service is rebuildable from BrainStore state and alignment journals.

## Async model

The public direction is hybrid:

```text
Tokio = network/control plane
Dedicated workers = storage and recall ownership
Rayon/compute pools = CPU-heavy scoring/chunking/rerank prep
Bounded channels = backpressure
```

Do not make every persistence or recall operation an unbounded async task.

## Target

The private qortex track should test whether dedicated recall service mode maintains or improves the target performance advantage over HTTP-only Qdrant-style deployment.
