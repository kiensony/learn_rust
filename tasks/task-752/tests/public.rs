use task_752::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_hash(Some("outside[red#blue]ignored")),
        None
    );
    assert_eq!(longest_field_with_last_tie_hash(Some("[42]")), Some(42));
}
