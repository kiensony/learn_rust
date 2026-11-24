use task_1068::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_slash(&"path/1/12"), 1);
    assert_eq!(first_nonempty_field_slash(&String::from("path/123")), 3);
}
