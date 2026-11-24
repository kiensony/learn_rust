use task_719::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_slash(Some("path/1/12")),
        Some(12)
    );
    assert_eq!(remainder_after_first_mark_slash(Some("path/123")), None);
}
