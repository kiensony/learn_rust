use task_2104::*;

#[test]
fn public_examples() {
    assert_eq!(sorted_keys_peaks(&[]), vec![]);
    assert_eq!(sorted_keys_peaks(&[4]), vec![4u32]);
}
