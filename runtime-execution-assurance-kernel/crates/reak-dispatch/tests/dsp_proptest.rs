use proptest::prelude::*;
use reak_dispatch::state_machine;
use reak_dispatch::DispatchState;

proptest! {
    #[test]
    fn terminal_outcomes_are_closed(outcome in prop::sample::select(&state_machine::TERMINAL_OUTCOMES)) {
        prop_assert!(state_machine::is_terminal_outcome(outcome));
    }

    #[test]
    fn skip_released_to_awaiting_is_illegal(from in prop_oneof![
        Just(DispatchState::TicketIssued),
        Just(DispatchState::Released),
        Just(DispatchState::AwaitingAck),
    ]) {
        if from != DispatchState::Released {
            prop_assert!(state_machine::is_illegal_lifecycle_transition(from, DispatchState::AwaitingAck)
                || from == DispatchState::AwaitingAck);
        }
    }
}

#[test]
fn all_terminal_outcomes_ack_once() {
    for outcome in state_machine::TERMINAL_OUTCOMES {
        assert!(state_machine::is_terminal_outcome(outcome));
    }
}
