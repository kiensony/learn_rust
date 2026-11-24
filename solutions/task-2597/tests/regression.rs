use task_2597::*;

#[test]
fn transfer_case() {
    assert_eq!(
        odd_element_proof_run_ledger(&[
            3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
        ]),
        vec![
            (7u32, 1usize),
            (13u32, 1usize),
            (16u32, 1usize),
            (27u32, 1usize),
            (32u32, 1usize),
            (9u32, 1usize),
            (1u32, 1usize),
            (22u32, 1usize),
            (10u32, 1usize),
            (4u32, 1usize)
        ]
    );
}
