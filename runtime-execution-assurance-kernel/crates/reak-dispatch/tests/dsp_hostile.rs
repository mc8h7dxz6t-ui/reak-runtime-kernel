mod dsp_fixtures;

use dsp_fixtures::wired_dispatch;
use reak_dispatch::{DispatchError, DispatchSpec};
use std::sync::Arc;
use std::thread;

#[test]
fn duplicate_ticket_rejected() {
    let w = wired_dispatch();
    let spec = DispatchSpec {
        commitment_id: w.binding.record.commitment_id.clone(),
        commitment_digest: w.binding.digest.clone(),
    };
    w.dispatch
        .issue_ticket(&w.commitment, &w.exposure, &w.policy, &w.ues, spec.clone(), 1)
        .unwrap();
    let err = w
        .dispatch
        .issue_ticket(&w.commitment, &w.exposure, &w.policy, &w.ues, spec, 2)
        .unwrap_err();
    assert_eq!(err, DispatchError::DuplicateDispatch);
}

#[test]
fn concurrent_issue_single_winner() {
    let w = Arc::new(wired_dispatch());
    let spec = DispatchSpec {
        commitment_id: w.binding.record.commitment_id.clone(),
        commitment_digest: w.binding.digest.clone(),
    };
    let mut handles = vec![];
    for i in 0..6 {
        let s = Arc::clone(&w);
        let spec = spec.clone();
        handles.push(thread::spawn(move || {
            s.dispatch.issue_ticket(
                &s.commitment,
                &s.exposure,
                &s.policy,
                &s.ues,
                spec,
                i as u64,
            )
        }));
    }
    let mut ok = 0;
    let mut dup = 0;
    for h in handles {
        match h.join().unwrap() {
            Ok(_) => ok += 1,
            Err(DispatchError::DuplicateDispatch) => dup += 1,
            Err(e) => panic!("{:?}", e),
        }
    }
    assert_eq!(ok, 1);
    assert_eq!(dup, 5);
}

#[test]
fn authority_revoked_before_issue() {
    let w = wired_dispatch();
    w.exposure
        .authority()
        .revoke("principal:effect.pay")
        .unwrap();
    let spec = DispatchSpec {
        commitment_id: w.binding.record.commitment_id.clone(),
        commitment_digest: w.binding.digest.clone(),
    };
    let err = w
        .dispatch
        .issue_ticket(&w.commitment, &w.exposure, &w.policy, &w.ues, spec, 1)
        .unwrap_err();
    assert_eq!(err, DispatchError::AuthorityRevoked);
}

#[test]
fn double_execute_rejected() {
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
    assert!(w.dispatch.execute_once(&ticket.ticket_id, 3).is_err());
}
