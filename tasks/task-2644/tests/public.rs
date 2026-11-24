use task_2644::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_nibble_swap_peaks(&[]), vec![]);
    assert_eq!(embedded_nibble_swap_peaks(&[4]), vec![64u32]);
}
