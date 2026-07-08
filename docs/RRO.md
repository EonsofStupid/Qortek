# Reason Ready Objects

A Reason Ready Object is a durable unit of context prepared for reasoning, retrieval, replay, and audit.

RROs exist because raw chat text is not enough. A vague operator request has to become something structured enough that a local brain store, vector recall engine, and mesh node can all act on it consistently.

## Public model

An RRO contains:

- stable ID
- workspace ID
- kind
- seed context
- claims
- evidence references
- boundary references
- graph edge references
- vector references
- lifecycle state
- timestamps

## Lifecycle

```text
Draft
  -> Declared
  -> VectorPending
  -> Vectorized
  -> Superseded / Archived
```

## RRO kinds

- `SeedChat`
- `Memory`
- `Plan`
- `Decision`
- `ToolTrace`
- `ForkSummary`

## Principle

RROs are the bridge between human vagueness and machine-operational context.
