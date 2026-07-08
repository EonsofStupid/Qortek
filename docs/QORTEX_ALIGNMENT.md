# Qortex Alignment

Qortek and Qortex are designed to work in parallel.

```text
Qortek BrainStore = durable truth
Qortex VectorRecall = vector retrieval engine
```

The public repo does not ship private qortex release code. It defines the contracts that allow Qortex, Qdrant, or another vector backend to plug into the RRD.

## Alignment states

Future alignment journals should track states such as:

- `PendingVectorization`
- `Vectorized`
- `VectorOutOfDate`
- `VectorDeleted`
- `RepairNeeded`

## Rule

The brain store wins. Vector state is rebuildable.

## Adapter boundary

Vector adapters must not leak backend-specific types into core RRO, store, fork, or mesh crates.
