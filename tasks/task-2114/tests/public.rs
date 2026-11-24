use task_2114::*;

#[test]
fn public_examples() {
    assert_eq!(priority_removal_peaks(&[]), vec![]);
    assert_eq!(priority_removal_peaks(&[4]), vec![4u32]);
}
