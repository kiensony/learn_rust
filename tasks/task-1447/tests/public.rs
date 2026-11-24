use task_1447::*;

#[test]
fn public_examples() {
    assert_eq!(
        elapsed_seconds_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![(0u32, 10usize)]
    );
    assert_eq!(elapsed_seconds_run_ledger(&[]), vec![]);
}
