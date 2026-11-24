use task_2697::*;

#[test]
fn public_examples() {
    assert_eq!(
        lending_first_byte_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (3u32, 1usize),
            (12u32, 1usize),
            (0u32, 1usize),
            (31u32, 1usize)
        ]
    );
    assert_eq!(lending_first_byte_run_ledger(&[]), vec![]);
}
