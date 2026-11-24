use task_2394::*;

#[test]
fn public_examples() {
    assert_eq!(last_change_age_peaks(&[]), vec![]);
    assert_eq!(last_change_age_peaks(&[4]), vec![1u32]);
}
