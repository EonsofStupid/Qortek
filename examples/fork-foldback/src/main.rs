use qortek_core::WorkspaceId;
use qortek_fork::{ConversationFork, Foldback};
use qortek_rro::{ReasonReadyObject, RroKind};

fn main() {
    let seed = ReasonReadyObject::new(
        WorkspaceId::new(),
        RroKind::SeedChat,
        "The operator has a vague branch-worthy request.",
    );

    let fork = ConversationFork::open(seed.id, "clarify persistence and vector alignment").commit();
    let foldback = Foldback::new(fork.id, "The branch resolved into a durable brain + vector recall alignment plan.", vec![seed.id]);

    println!("fork: {fork:#?}");
    println!("foldback: {foldback:#?}");
}
