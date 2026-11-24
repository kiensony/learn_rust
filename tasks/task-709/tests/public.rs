use task_709::*;

#[test]
fn public_examples() {
    assert_eq!(marks_removed_at_both_ends_colon(Some("hi:kit")), None);
    assert_eq!(marks_removed_at_both_ends_colon(Some(":9")), Some(9));
}
