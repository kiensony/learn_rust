use task_265::*;

#[test]
fn public_examples() {
    assert_eq!(last_field_hash("cat"), "cat");
    assert_eq!(last_field_hash("#hi#"), "");
}
