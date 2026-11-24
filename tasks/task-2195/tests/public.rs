use task_2195::*;

#[test]
fn public_examples() {
    assert_eq!(duplicate_insertion_cost_changes(&[]), vec![]);
    assert_eq!(duplicate_insertion_cost_changes(&[4]), vec![1u32]);
}
