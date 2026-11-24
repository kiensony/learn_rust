use task_1166::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_slash("path/1/12", "backup"),
        "12"
    );
    assert_eq!(
        remainder_after_first_mark_slash("path/123", "backup"),
        "backup"
    );
}
