use task_2697::*;

#[test]
fn transfer_case() {
    assert_eq!(
        lending_first_byte_run_ledger(&[
            3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
        ]),
        vec![
            (3u32, 1usize),
            (12u32, 1usize),
            (0u32, 1usize),
            (31u32, 1usize),
            (26u32, 1usize),
            (21u32, 1usize),
            (6u32, 1usize)
        ]
    );
}
