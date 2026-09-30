mod dsp_fixtures;

use dsp_fixtures::wired_dispatch;
use reak_dispatch::{DispatchOutcome, DispatchSpec};

#[test]
fn issue_execute_ack_success() {
    let w = wired_dispatch();
    let spec = DispatchSpec {
        commitment_id: w.binding.record.commitment_id.clone(),
        commitment_digest: w.binding.digest.clone(),
    };
    let ticket = w
        .dispatch
        .issue_ticket(
            &w.commitment,
            &w.exposure,
            &w.policy,
            &w.ues,
            spec,
            1,
        )
        .unwrap();
    w.dispatch.execute_once(&ticket.ticket_id, 2).unwrap();
    let outcome = w
        .dispatch
        .ack_or_nack(&ticket.ticket_id, DispatchOutcome::Success, 3)
        .unwrap();
    assert_eq!(outcome.outcome, DispatchOutcome::Success);
}

#[test]
fn unknown_does_not_allow_second_execute() {
    let w = wired_dispatch();
    let spec = DispatchSpec {
        commitment_id: w.binding.record.commitment_id.clone(),
        commitment_digest: w.binding.digest.clone(),
    };
    let ticket = w
        .dispatch
        .issue_ticket(
            &w.commitment,
            &w.exposure,
            &w.policy,
            &w.ues,
            spec,
            1,
        )
        .unwrap();
    w.dispatch.execute_once(&ticket.ticket_id, 2).unwrap();
    w.dispatch
        .ack_or_nack(&ticket.ticket_id, DispatchOutcome::Unknown, 3)
        .unwrap();
    assert!(w.dispatch.execute_once(&ticket.ticket_id, 4).is_err());
}
