//! Surreal-compatible brain-store adapter placeholder.
//!
//! This public crate is the boundary for Surreal-style persistence integration.
//! It should implement `qortek_store::BrainStore` while preserving Qortek's own
//! durable brain model and alignment journal semantics.

pub const ADAPTER_NAME: &str = "surreal";
