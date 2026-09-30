use reak_types::{StageId, TenantId};
use reak_ues::{StageBudget, UncertaintyDelta, UncertaintyLedger, UesError};

#[test]
fn consume_without_declare_fails_closed() {
    let ledger = UncertaintyLedger::new(TenantId::parse("t").unwrap());
    let err = ledger
        .consume_budget(StageId::new(1), UncertaintyDelta { eliminable_units: 1 })
        .unwrap_err();
    assert_eq!(err, UesError::BudgetNotDeclared);
}

#[test]
fn overrun_records_violation_and_fails() {
    let ledger = UncertaintyLedger::new(TenantId::parse("t").unwrap());
    ledger
        .declare_budget(StageBudget {
            stage: StageId::new(2),
            max_eliminable_units: 2,
        })
        .unwrap();
    ledger
        .consume_budget(StageId::new(2), UncertaintyDelta { eliminable_units: 2 })
        .unwrap();
    let err = ledger
        .consume_budget(StageId::new(2), UncertaintyDelta { eliminable_units: 1 })
        .unwrap_err();
    assert_eq!(err, UesError::BudgetExhausted);
}
