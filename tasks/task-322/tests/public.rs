use task_322::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_slash(String::from("path/1/12")),
        "1/12"
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(String::from("path/123")),
        "123"
    );
}
