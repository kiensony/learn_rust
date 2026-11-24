use task_1247::*;

#[test]
fn public_examples() {
    assert_eq!(
        read_chunks_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![(3u32, 3usize), (1u32, 1usize)]
    );
    assert_eq!(read_chunks_run_ledger(&[]), vec![]);
}
