use task_2141::*;

#[test]
fn public_examples() {
    assert_eq!(first_occurrences_trace(&[]), vec![]);
    assert_eq!(first_occurrences_trace(&[4]), vec![4u32]);
}
