use task_2194::*;

#[test]
fn public_examples() {
    assert_eq!(duplicate_insertion_cost_peaks(&[]), vec![]);
    assert_eq!(duplicate_insertion_cost_peaks(&[4]), vec![1u32]);
}
