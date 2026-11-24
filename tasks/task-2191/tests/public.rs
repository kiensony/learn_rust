use task_2191::*;

#[test]
fn public_examples() {
    assert_eq!(duplicate_insertion_cost_trace(&[]), vec![]);
    assert_eq!(duplicate_insertion_cost_trace(&[4]), vec![1u32]);
}
