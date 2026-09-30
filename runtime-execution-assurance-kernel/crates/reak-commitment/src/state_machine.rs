use crate::model::CommitmentStatus;

/// Legal status transitions for commitment lifecycle (void only after bind).
pub fn can_transition(from: CommitmentStatus, to: CommitmentStatus) -> bool {
    matches!(
        (from, to),
        (CommitmentStatus::Bound, CommitmentStatus::Voided)
    )
}

pub fn is_terminal(status: CommitmentStatus) -> bool {
    status == CommitmentStatus::Voided
}
