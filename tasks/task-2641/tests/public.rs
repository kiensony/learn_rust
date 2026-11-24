use task_2641::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_nibble_swap_trace(&[]), vec![]);
    assert_eq!(embedded_nibble_swap_trace(&[4]), vec![64u32]);
}
