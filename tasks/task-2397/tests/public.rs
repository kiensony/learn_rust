use task_2397::*;

#[test]
fn public_examples() {
    assert_eq!(
        single_pass_prefix_minimum_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![(3u32, 6usize), (0u32, 4usize)]
    );
    assert_eq!(single_pass_prefix_minimum_run_ledger(&[]), vec![]);
}
