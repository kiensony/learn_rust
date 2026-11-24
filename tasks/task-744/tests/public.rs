use task_744::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_hash(Some("outside[red#blue]ignored")), None);
    assert_eq!(leading_field_hash(Some("[42]")), Some(42));
}
