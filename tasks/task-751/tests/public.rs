use task_751::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_nonempty_field_hash(Some("outside[red#blue]ignored")),
        None
    );
    assert_eq!(first_nonempty_field_hash(Some("[42]")), Some(42));
}
