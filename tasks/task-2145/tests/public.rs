use task_2145::*;

#[test]
fn public_examples() {
    assert_eq!(first_occurrences_changes(&[]), vec![]);
    assert_eq!(first_occurrences_changes(&[4]), vec![4u32]);
}
