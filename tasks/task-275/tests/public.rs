use task_275::*;

#[test]
fn public_examples() {
    assert_eq!(last_field_star("cat"), "cat");
    assert_eq!(last_field_star("*hi*"), "");
}
