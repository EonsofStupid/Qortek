//! Preflight planning for resource, storage, vector, and mesh routing.

use qortek_mesh::TransportProfile;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceProfile {
    pub cpu_cores: usize,
    pub memory_gb: usize,
    pub has_blackwell_fabric: bool,
    pub preferred_transport: TransportProfile,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PreflightPlan {
    pub embedded_brain: bool,
    pub embedded_vector: bool,
    pub remote_estate_allowed: bool,
    pub transport_profile: TransportProfile,
}

impl PreflightPlan {
    #[must_use]
    pub fn from_profile(profile: &ResourceProfile) -> Self {
        Self {
            embedded_brain: true,
            embedded_vector: true,
            remote_estate_allowed: matches!(
                profile.preferred_transport,
                TransportProfile::FabricAdjacent
                    | TransportProfile::RemoteHttp
                    | TransportProfile::RemoteQuic
            ),
            transport_profile: profile.preferred_transport.clone(),
        }
    }
}
