use task_745::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_hash(Some("outside[red#blue]ignored")), None);
    assert_eq!(closing_field_hash(Some("[42]")), Some(42));
}
