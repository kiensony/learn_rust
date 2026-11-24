use task_748::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_hash(Some("outside[red#blue]ignored")),
        None
    );
    assert_eq!(marks_removed_at_both_ends_hash(Some("[42]")), Some(42));
}
