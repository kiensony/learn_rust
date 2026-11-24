use task_747::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_hash(Some("outside[red#blue]ignored")),
        None
    );
    assert_eq!(closing_marks_removed_hash(Some("[42]")), Some(42));
}
