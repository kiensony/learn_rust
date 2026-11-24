use task_2142::*;

#[test]
fn public_examples() {
    assert_eq!(first_occurrences_total(&[]), 0);
    assert_eq!(first_occurrences_total(&[4]), 4);
}
