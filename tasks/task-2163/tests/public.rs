use task_2163::*;

#[test]
fn public_examples() {
    assert_eq!(sorted_insertion_moves_bounds(&[]), None);
    assert_eq!(sorted_insertion_moves_bounds(&[4]), Some((0u32, 0u32)));
}
