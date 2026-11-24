use task_1404::*;

#[test]
fn public_examples() {
    assert_eq!(millisecond_remainder_peaks(&[]), vec![]);
    assert_eq!(millisecond_remainder_peaks(&[4]), vec![68u32]);
}
