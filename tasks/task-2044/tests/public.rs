use task_2044::*;

#[test]
fn public_examples() {
    assert_eq!(polled_costs_peaks(&[]), vec![]);
    assert_eq!(polled_costs_peaks(&[4]), vec![1u32]);
}
