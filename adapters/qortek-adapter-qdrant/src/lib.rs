//! Public Qdrant adapter placeholder.
//!
//! The initial public repo keeps this crate dependency-light. The real adapter
//! should implement `qortek_vector::VectorRecall` without leaking backend types
//! into Qortek's core contracts.

pub const ADAPTER_NAME: &str = "qdrant";
