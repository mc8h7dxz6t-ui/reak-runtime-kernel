use crate::model::{DispatchOutcome, DispatchState};

/// Terminal outcomes — no further external effect is permitted for this ticket.
pub fn is_terminal_outcome(outcome: DispatchOutcome) -> bool {
    matches!(
        outcome,
        DispatchOutcome::Success
            | DispatchOutcome::Unknown
            | DispatchOutcome::Failed
            | DispatchOutcome::Cancelled
            | DispatchOutcome::Timeout
    )
}

/// Legal transitions for dispatch lifecycle (no implicit edges).
pub fn can_transition(from: DispatchState, to: DispatchState) -> bool {
    matches!(
        (from, to),
        (DispatchState::TicketIssued, DispatchState::Released)
            | (DispatchState::Released, DispatchState::AwaitingAck)
            | (DispatchState::AwaitingAck, DispatchState::AwaitingAck)
    )
}

/// All terminal outcomes (immutable once recorded).
pub const TERMINAL_OUTCOMES: [DispatchOutcome; 5] = [
    DispatchOutcome::Success,
    DispatchOutcome::Unknown,
    DispatchOutcome::Failed,
    DispatchOutcome::Cancelled,
    DispatchOutcome::Timeout,
];

/// Illegal lifecycle transitions (explicit deny list for verification tests).
pub fn is_illegal_lifecycle_transition(from: DispatchState, to: DispatchState) -> bool {
    !can_transition(from, to) && from != to
}
