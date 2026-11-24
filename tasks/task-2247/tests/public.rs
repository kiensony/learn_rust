use task_2247::*;

#[test]
fn public_examples() {
    assert_eq!(
        borrowed_offsets_run_ledger("a\nblue\nblue\r\nz\n"),
        vec![
            (1u32, 1usize),
            (6u32, 1usize),
            (12u32, 1usize),
            (14u32, 1usize)
        ]
    );
    assert_eq!(borrowed_offsets_run_ledger(""), vec![]);
}
