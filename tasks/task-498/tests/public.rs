use task_498::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_hash(("outside[red#blue]ignored", "x")), 3);
    assert_eq!(leading_field_hash(("unclosed[cat", "x")), 0);
}
