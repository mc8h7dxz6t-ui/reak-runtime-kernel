use reak_dispatch::state_machine;
use reak_dispatch::{DispatchOutcome, DispatchState};

#[test]
fn legal_transitions_documented() {
    assert!(state_machine::can_transition(
        DispatchState::TicketIssued,
        DispatchState::Released
    ));
    assert!(state_machine::can_transition(
        DispatchState::Released,
        DispatchState::AwaitingAck
    ));
    assert!(!state_machine::can_transition(
        DispatchState::TicketIssued,
        DispatchState::AwaitingAck
    ));
    assert!(state_machine::can_transition(
        DispatchState::AwaitingAck,
        DispatchState::AwaitingAck
    ));
}

#[test]
fn illegal_transitions_rejected() {
    assert!(state_machine::is_illegal_lifecycle_transition(
        DispatchState::AwaitingAck,
        DispatchState::TicketIssued
    ));
    assert!(state_machine::is_illegal_lifecycle_transition(
        DispatchState::TicketIssued,
        DispatchState::TicketIssued
    ) == false);
}

#[test]
fn every_listed_outcome_is_terminal() {
    for o in state_machine::TERMINAL_OUTCOMES {
        assert!(state_machine::is_terminal_outcome(o));
    }
    assert!(state_machine::is_terminal_outcome(DispatchOutcome::Success));
}
