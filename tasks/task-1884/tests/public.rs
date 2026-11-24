use task_1884::*;

#[test]
fn public_examples() {
    assert_eq!(unique_owner_state_peaks(&[]), vec![]);
    assert_eq!(unique_owner_state_peaks(&[4]), vec![1u32]);
}
