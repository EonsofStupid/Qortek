# Brain Store

The BrainStore is the durable persistence boundary for Qortek.

It is not optional infrastructure. It is the persistent AI brain that keeps Qortex/vector recall aligned with real state.

## Responsibilities

- persist RROs
- persist memory declarations
- persist fork state
- persist foldbacks
- append audit events
- preserve lineage
- support vector repair/rebuild

## Backends

The public trait can be implemented by:

- in-memory adapter for examples/tests
- Surreal-compatible adapter
- RocksDB/native adapter
- remote estate node adapter

## Alignment principle

The BrainStore owns truth. Vector state is derived and repairable.
