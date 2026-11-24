use task_1155::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_colon("hi:kit", "backup"),
        "hi:kit"
    );
    assert_eq!(marks_removed_at_both_ends_colon(":é:a", "backup"), "é:a");
}
