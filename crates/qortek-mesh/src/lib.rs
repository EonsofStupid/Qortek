//! AI-to-AI mesh contracts.

use async_trait::async_trait;
use qortek_core::{NodeId, Result};
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait MeshTransport: Send + Sync {
    async fn advertise(&self, node: NodeDescriptor) -> Result<()>;
    async fn discover(&self, filter: NodeFilter) -> Result<Vec<NodeDescriptor>>;
    async fn send(&self, envelope: MeshEnvelope) -> Result<MeshReply>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeDescriptor {
    pub id: NodeId,
    pub name: String,
    pub capabilities: Vec<String>,
    pub transport_profile: TransportProfile,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeFilter {
    pub required_capability: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TransportProfile {
    LocalEmbedded,
    FabricAdjacent,
    RemoteHttp,
    RemoteQuic,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MeshEnvelope {
    pub target: NodeId,
    pub capability: String,
    pub payload_json: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MeshReply {
    pub payload_json: serde_json::Value,
}
