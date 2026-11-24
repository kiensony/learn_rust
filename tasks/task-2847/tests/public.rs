use task_2847::*;

#[test]
fn public_examples() {
    assert_eq!(
        service_validation_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![(1u32, 6usize), (0u32, 1usize), (1u32, 3usize)]
    );
    assert_eq!(service_validation_run_ledger(&[]), vec![]);
}
