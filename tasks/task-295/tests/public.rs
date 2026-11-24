use task_295::*;

#[test]
fn public_examples() {
    assert_eq!(last_field_plus("cat"), "cat");
    assert_eq!(last_field_plus("+hi+"), "");
}
