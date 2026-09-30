mod dsp_fixtures;

use dsp_fixtures::wired_dispatch;
use reak_dispatch::{DispatchOutcome, DispatchSpec};

#[test]
fn recover_after_crash_before_send_allows_single_release() {
    let w = wired_dispatch();
    let spec = DispatchSpec {
        commitment_id: w.binding.record.commitment_id.clone(),
        commitment_digest: w.binding.digest.clone(),
    };
    let ticket = w
        .dispatch
        .issue_ticket(&w.commitment, &w.exposure, &w.policy, &w.ues, spec, 1)
        .unwrap();
    let n = w.dispatch.recover_from_log().unwrap();
    assert_eq!(n, 1);
    assert!(w.dispatch.execute_once(&ticket.ticket_id, 2).is_ok());
}

#[test]
fn recover_after_crash_after_send_blocks_second_release() {
    let w = wired_dispatch();
    let spec = DispatchSpec {
        commitment_id: w.binding.record.commitment_id.clone(),
        commitment_digest: w.binding.digest.clone(),
    };
    let ticket = w
        .dispatch
        .issue_ticket(&w.commitment, &w.exposure, &w.policy, &w.ues, spec, 1)
        .unwrap();
    w.dispatch.execute_once(&ticket.ticket_id, 2).unwrap();
    let n = w.dispatch.recover_from_log().unwrap();
    assert_eq!(n, 2);
    assert!(w.dispatch.execute_once(&ticket.ticket_id, 3).is_err());
}

#[test]
fn recover_idempotent_after_terminal_outcome() {
    let w = wired_dispatch();
    let spec = DispatchSpec {
        commitment_id: w.binding.record.commitment_id.clone(),
        commitment_digest: w.binding.digest.clone(),
    };
    let ticket = w
        .dispatch
        .issue_ticket(&w.commitment, &w.exposure, &w.policy, &w.ues, spec, 1)
        .unwrap();
    w.dispatch.execute_once(&ticket.ticket_id, 2).unwrap();
    w.dispatch
        .ack_or_nack(&ticket.ticket_id, DispatchOutcome::Timeout, 3)
        .unwrap();
    let n1 = w.dispatch.recover_from_log().unwrap();
    let n2 = w.dispatch.recover_from_log().unwrap();
    assert_eq!(n1, n2);
    assert_eq!(n1, 3);
}
