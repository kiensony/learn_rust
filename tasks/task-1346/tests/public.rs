use task_1346::*;

#[test]
fn public_examples() {
    assert_eq!(
        decimal_fields_threshold_crossing("4,9,2,2,0", 0),
        Some((0, 4))
    );
    assert_eq!(decimal_fields_threshold_crossing("", 0), None);
}
