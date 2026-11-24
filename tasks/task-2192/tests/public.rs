use task_2192::*;

#[test]
fn public_examples() {
    assert_eq!(duplicate_insertion_cost_total(&[]), 0);
    assert_eq!(duplicate_insertion_cost_total(&[4]), 1);
}
