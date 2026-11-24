use task_900::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_hash("outside[red#blue]ignored"), 5);
    assert_eq!(first_nonempty_field_hash("unclosed[cat"), 2);
}
