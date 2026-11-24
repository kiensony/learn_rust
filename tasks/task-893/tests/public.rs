use task_893::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_hash("outside[red#blue]ignored"), 5);
    assert_eq!(leading_field_hash("unclosed[cat"), 2);
}
