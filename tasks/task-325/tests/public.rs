use task_325::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_slash(String::from("path/1/12")), "1");
    assert_eq!(first_nonempty_field_slash(String::from("path/123")), "123");
}
