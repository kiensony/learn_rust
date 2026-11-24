use task_2144::*;

#[test]
fn public_examples() {
    assert_eq!(first_occurrences_peaks(&[]), vec![]);
    assert_eq!(first_occurrences_peaks(&[4]), vec![4u32]);
}
