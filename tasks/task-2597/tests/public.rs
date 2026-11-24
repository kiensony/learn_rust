use task_2597::*;

#[test]
fn public_examples() {
    assert_eq!(
        odd_element_proof_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (7u32, 1usize),
            (13u32, 1usize),
            (16u32, 1usize),
            (27u32, 1usize),
            (32u32, 1usize)
        ]
    );
    assert_eq!(odd_element_proof_run_ledger(&[]), vec![]);
}
