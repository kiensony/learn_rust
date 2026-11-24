use task_156::*;

#[test]
fn transfer_case() {
    assert_eq!(saturating_debit_10(100, 100), 90);
}
