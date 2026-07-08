# Architecture

Qortek is the durable brain substrate for the Reason Ready Daemon.

It keeps persistent truth separate from vector recall while requiring both to remain aligned.

```text
Operator prompt
  -> seed RRO
  -> preflight plan
  -> BrainStore write
  -> VectorRecall indexing
  -> journal / event append
  -> fork if ambiguity requires it
  -> foldback into seed context
```

## Layers

```text
Qortek RRD
├─ qortek-rro        Reason Ready Object model
├─ qortek-store      durable brain-store traits
├─ qortek-vector     vector recall traits
├─ qortek-fork       branch lifecycle and foldback
├─ qortek-mesh       AI-to-AI transport contracts
├─ qortek-preflight  local/remote resource routing
└─ adapters          memory, Qdrant/Qortex, Surreal-compatible backends
```

## Brain store vs vector store

The brain store is the source of truth.

It owns:

- RROs
- memories
- fork state
- graph edges
- plans
- decisions
- audit events
- workspace state
- vector alignment journal entries

The vector store is rebuildable recall infrastructure.

It owns:

- dense vectors
- sparse vectors
- hybrid retrieval indexes
- ANN search structures
- collection-level recall tuning

## Alignment rule

For any memory-bearing object:

```text
brain object version
  -> vector document version
  -> alignment status
```

If drift exists, the brain store wins and vector state is repaired.
