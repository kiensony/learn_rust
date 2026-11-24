use task_2193::*;

#[test]
fn public_examples() {
    assert_eq!(duplicate_insertion_cost_bounds(&[]), None);
    assert_eq!(duplicate_insertion_cost_bounds(&[4]), Some((1u32, 1u32)));
}
