use task_2247::*;

#[test]
fn transfer_case() {
    assert_eq!(
        borrowed_offsets_run_ledger("a\nblue\nblue\r\nz\n a\nblue\nblue\r\nz\n"),
        vec![
            (1u32, 1usize),
            (6u32, 1usize),
            (12u32, 1usize),
            (14u32, 1usize),
            (17u32, 1usize),
            (22u32, 1usize),
            (28u32, 1usize),
            (30u32, 1usize)
        ]
    );
}
