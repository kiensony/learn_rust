use task_2397::*;

#[test]
fn transfer_case() {
    assert_eq!(
        single_pass_prefix_minimum_run_ledger(&[
            3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
        ]),
        vec![(3u32, 6usize), (0u32, 14usize)]
    );
}
