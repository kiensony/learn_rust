use task_2024::*;

#[test]
fn public_examples() {
    assert_eq!(polled_groups_peaks(&[]), vec![]);
    assert_eq!(polled_groups_peaks(&[4]), vec![1u32]);
}
