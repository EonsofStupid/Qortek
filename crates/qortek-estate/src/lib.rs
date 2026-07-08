//! Estate foundation types.
//!
//! An estate is a standardized place where Clyffy can arrive, preflight,
//! load local MCP/context, work, cache, remember, and leave replay evidence.

use chrono::{DateTime, Utc};
use qortek_core::{NodeId, ProjectScope, Result};
use qortek_mesh::TransportProfile;
use serde::{Deserialize, Serialize};

pub const ESTATE_ROOT: &str = ".clyffy";
pub const ESTATE_MANIFEST: &str = ".clyffy/estate.json";
pub const ENDPOINT_MANIFEST: &str = ".clyffy/endpoint.json";
pub const MCP_MANIFEST: &str = ".clyffy/mcp/manifest.json";
pub const LAST_TOUCHED_LEDGER: &str = ".clyffy/history/last_touched.jsonl";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EstateDomain {
    Identity,
    Policy,
    Memory,
    Recall,
    Connectome,
    Ingestion,
    Cache,
    History,
    Sessions,
    Replays,
    Tools,
    Artifacts,
    Telemetry,
    Sync,
}

impl EstateDomain {
    #[must_use]
    pub fn folder(&self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::Policy => "policies",
            Self::Memory => "memory",
            Self::Recall => "qortex",
            Self::Connectome => "connectome",
            Self::Ingestion => "ingestion",
            Self::Cache => "cache",
            Self::History => "history",
            Self::Sessions => "sessions",
            Self::Replays => "replays",
            Self::Tools => "tools",
            Self::Artifacts => "artifacts",
            Self::Telemetry => "telemetry",
            Self::Sync => "sync",
        }
    }

    #[must_use]
    pub fn standard() -> Vec<Self> {
        vec![
            Self::Identity,
            Self::Policy,
            Self::Memory,
            Self::Recall,
            Self::Connectome,
            Self::Ingestion,
            Self::Cache,
            Self::History,
            Self::Sessions,
            Self::Replays,
            Self::Tools,
            Self::Artifacts,
            Self::Telemetry,
            Self::Sync,
        ]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EstateManifest {
    pub estate_id: NodeId,
    pub name: String,
    pub purpose: String,
    pub scope: ProjectScope,
    pub domains: Vec<EstateDomain>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl EstateManifest {
    #[must_use]
    pub fn new(name: impl Into<String>, purpose: impl Into<String>, scope: ProjectScope) -> Self {
        let now = Utc::now();
        Self {
            estate_id: NodeId::new(),
            name: name.into(),
            purpose: purpose.into(),
            scope,
            domains: EstateDomain::standard(),
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EndpointManifest {
    pub estate_id: NodeId,
    pub endpoint_url: Option<String>,
    pub transport_profile: TransportProfile,
    pub capabilities: Vec<String>,
    pub policy_profile: String,
    pub mcp_manifest_path: String,
    pub connectome_version: String,
    pub qortex_version: String,
    pub last_seen: Option<DateTime<Utc>>,
    pub last_touched: Option<DateTime<Utc>>,
}

impl EndpointManifest {
    #[must_use]
    pub fn local_embedded(estate_id: NodeId) -> Self {
        Self {
            estate_id,
            endpoint_url: None,
            transport_profile: TransportProfile::LocalEmbedded,
            capabilities: vec![
                "brain-store".to_string(),
                "connectome".to_string(),
                "mcp".to_string(),
            ],
            policy_profile: "local-default-deny".to_string(),
            mcp_manifest_path: MCP_MANIFEST.to_string(),
            connectome_version: env!("CARGO_PKG_VERSION").to_string(),
            qortex_version: "private-track".to_string(),
            last_seen: None,
            last_touched: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpManifest {
    pub estate_id: NodeId,
    pub allowed_roots: Vec<String>,
    pub denied_paths: Vec<String>,
    pub tools: Vec<McpToolRef>,
    pub context_sources: Vec<String>,
}

impl McpManifest {
    #[must_use]
    pub fn safe_default(estate_id: NodeId) -> Self {
        Self {
            estate_id,
            allowed_roots: vec![".".to_string()],
            denied_paths: vec![
                ".git".to_string(),
                ".env".to_string(),
                ".env.*".to_string(),
                "target".to_string(),
            ],
            tools: Vec::new(),
            context_sources: vec!["files".to_string(), "history".to_string(), "replays".to_string()],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpToolRef {
    pub name: String,
    pub command: String,
    pub description: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LastTouchedActor {
    Clyffy,
    Operator,
    Tool,
    RemoteAgent,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LastTouchedOperation {
    Read,
    Write,
    Ingest,
    Recall,
    Repair,
    Sync,
    Preflight,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LastTouchedEntry {
    pub actor: LastTouchedActor,
    pub domain: EstateDomain,
    pub path: String,
    pub operation: LastTouchedOperation,
    pub timestamp: DateTime<Utc>,
    pub replay_id: Option<String>,
}

impl LastTouchedEntry {
    #[must_use]
    pub fn now(
        actor: LastTouchedActor,
        domain: EstateDomain,
        path: impl Into<String>,
        operation: LastTouchedOperation,
    ) -> Self {
        Self {
            actor,
            domain,
            path: path.into(),
            operation,
            timestamp: Utc::now(),
            replay_id: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PreflightDecision {
    Allow,
    Deny,
    Degraded,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EstatePreflightReport {
    pub estate_id: NodeId,
    pub decision: PreflightDecision,
    pub checks: Vec<PreflightCheck>,
    pub selected_transport: TransportProfile,
    pub generated_at: DateTime<Utc>,
}

impl EstatePreflightReport {
    #[must_use]
    pub fn from_manifests(estate: &EstateManifest, endpoint: &EndpointManifest) -> Self {
        let mut checks = Vec::new();
        checks.push(PreflightCheck::pass("estate manifest loaded"));
        checks.push(PreflightCheck::pass("endpoint manifest loaded"));

        if estate.estate_id == endpoint.estate_id {
            checks.push(PreflightCheck::pass("estate id matches endpoint"));
        } else {
            checks.push(PreflightCheck::fail("estate id mismatch"));
        }

        if endpoint.policy_profile.is_empty() {
            checks.push(PreflightCheck::fail("policy profile missing"));
        } else {
            checks.push(PreflightCheck::pass("policy profile present"));
        }

        if endpoint.capabilities.is_empty() {
            checks.push(PreflightCheck::degraded("no capabilities advertised"));
        } else {
            checks.push(PreflightCheck::pass("capabilities advertised"));
        }

        let decision = if checks.iter().any(|check| check.decision == PreflightDecision::Deny) {
            PreflightDecision::Deny
        } else if checks
            .iter()
            .any(|check| check.decision == PreflightDecision::Degraded)
        {
            PreflightDecision::Degraded
        } else {
            PreflightDecision::Allow
        };

        Self {
            estate_id: estate.estate_id,
            decision,
            checks,
            selected_transport: endpoint.transport_profile.clone(),
            generated_at: Utc::now(),
        }
    }

    #[must_use]
    pub fn allowed(&self) -> bool {
        self.decision == PreflightDecision::Allow
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PreflightCheck {
    pub name: String,
    pub decision: PreflightDecision,
    pub note: Option<String>,
}

impl PreflightCheck {
    #[must_use]
    pub fn pass(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            decision: PreflightDecision::Allow,
            note: None,
        }
    }

    #[must_use]
    pub fn fail(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            decision: PreflightDecision::Deny,
            note: None,
        }
    }

    #[must_use]
    pub fn degraded(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            decision: PreflightDecision::Degraded,
            note: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EstateKitFile {
    pub path: String,
    pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EstateKit {
    pub estate: EstateManifest,
    pub endpoint: EndpointManifest,
    pub mcp: McpManifest,
}

impl EstateKit {
    #[must_use]
    pub fn new(name: impl Into<String>, purpose: impl Into<String>, scope: ProjectScope) -> Self {
        let estate = EstateManifest::new(name, purpose, scope);
        let endpoint = EndpointManifest::local_embedded(estate.estate_id);
        let mcp = McpManifest::safe_default(estate.estate_id);
        Self {
            estate,
            endpoint,
            mcp,
        }
    }

    pub fn render_files(&self) -> Result<Vec<EstateKitFile>> {
        let mut files = Vec::new();
        files.push(EstateKitFile {
            path: ESTATE_MANIFEST.to_string(),
            content: serde_json::to_string_pretty(&self.estate)
                .map_err(|err| qortek_core::QortekError::Serialization(err.to_string()))?,
        });
        files.push(EstateKitFile {
            path: ENDPOINT_MANIFEST.to_string(),
            content: serde_json::to_string_pretty(&self.endpoint)
                .map_err(|err| qortek_core::QortekError::Serialization(err.to_string()))?,
        });
        files.push(EstateKitFile {
            path: MCP_MANIFEST.to_string(),
            content: serde_json::to_string_pretty(&self.mcp)
                .map_err(|err| qortek_core::QortekError::Serialization(err.to_string()))?,
        });
        files.push(EstateKitFile {
            path: LAST_TOUCHED_LEDGER.to_string(),
            content: String::new(),
        });

        for domain in EstateDomain::standard() {
            files.push(EstateKitFile {
                path: format!("{ESTATE_ROOT}/{}/.keep", domain.folder()),
                content: String::new(),
            });
        }

        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qortek_core::{ProjectSlug, WorkspaceId};

    #[test]
    fn estate_kit_renders_standard_files() {
        let scope = ProjectScope::new(
            WorkspaceId::new(),
            ProjectSlug::new("estate-test").expect("valid slug"),
        );
        let kit = EstateKit::new("Estate Test", "prove standardized estate shape", scope);
        let files = kit.render_files().expect("render files");
        let paths = files.iter().map(|file| file.path.as_str()).collect::<Vec<_>>();

        assert!(paths.contains(&ESTATE_MANIFEST));
        assert!(paths.contains(&ENDPOINT_MANIFEST));
        assert!(paths.contains(&MCP_MANIFEST));
        assert!(paths.contains(&LAST_TOUCHED_LEDGER));
        assert!(paths.contains(&".clyffy/connectome/.keep"));
        assert!(paths.contains(&".clyffy/qortex/.keep"));
        assert!(paths.contains(&".clyffy/replays/.keep"));
    }

    #[test]
    fn preflight_allows_matching_estate_and_endpoint() {
        let scope = ProjectScope::new(
            WorkspaceId::new(),
            ProjectSlug::new("estate-test").expect("valid slug"),
        );
        let kit = EstateKit::new("Estate Test", "preflight", scope);
        let report = EstatePreflightReport::from_manifests(&kit.estate, &kit.endpoint);
        assert!(report.allowed());
        assert_eq!(report.decision, PreflightDecision::Allow);
    }
}
