use proptest::prelude::*;
use reak_truth::compute_conclusion;
use tru_fixtures::{admissible, reconciliation_record};

mod tru_fixtures;

proptest! {
    #[test]
    fn unknown_reconciliation_never_established_success(
        i in 0u8..20
    ) {
        let rec_id = format!("rec-prop-{}", i);
        let inp = reak_truth::TruthDerivationInput {
            reconciliation: reconciliation_record(
                &rec_id,
                "dsp-prop",
                reak_reconciliation::ReconciliationOutcome::ReconciledUnknown,
            ),
            admissibility: admissible(),
        };
        let (conclusion, _) = compute_conclusion(&inp).unwrap();
        prop_assert_ne!(conclusion, reak_truth::TruthConclusion::EstablishedSuccess);
    }
}
