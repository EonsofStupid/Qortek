use qortek_core::WorkspaceId;
use qortek_rro::{ReasonReadyObject, RroKind};

fn main() {
    let rro = ReasonReadyObject::new(
        WorkspaceId::new(),
        RroKind::SeedChat,
        "Build a durable context moat for a vague operator request.",
    );

    println!("{}", serde_json::to_string_pretty(&rro).unwrap());
}
