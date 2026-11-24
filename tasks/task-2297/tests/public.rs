use task_2297::*;

#[test]
fn public_examples() {
    assert_eq!(
        hot_code_addresses_run_ledger(&[9, 2, 2, 8, 0, 5]),
        vec![(2u32, 1usize)]
    );
    assert_eq!(hot_code_addresses_run_ledger(&[]), vec![]);
}
