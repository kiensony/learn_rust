use task_1247::*;

#[test]
fn transfer_case() {
    assert_eq!(
        read_chunks_run_ledger(&[
            3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
        ]),
        vec![(3u32, 6usize), (2u32, 1usize)]
    );
}
