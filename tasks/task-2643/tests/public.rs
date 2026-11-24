use task_2643::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_nibble_swap_bounds(&[]), None);
    assert_eq!(embedded_nibble_swap_bounds(&[4]), Some((64u32, 64u32)));
}
