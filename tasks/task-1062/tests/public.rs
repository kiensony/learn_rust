use task_1062::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_slash(&"path/1/12"), 2);
    assert_eq!(closing_field_slash(&String::from("path/123")), 3);
}
