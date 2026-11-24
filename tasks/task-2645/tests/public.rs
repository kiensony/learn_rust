use task_2645::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_nibble_swap_changes(&[]), vec![]);
    assert_eq!(embedded_nibble_swap_changes(&[4]), vec![64u32]);
}
