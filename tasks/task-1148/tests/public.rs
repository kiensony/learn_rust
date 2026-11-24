use task_1148::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_hash(&"outside[red#blue]ignored"), 3);
    assert_eq!(first_nonempty_field_hash(&"unclosed[cat"), 0);
}
