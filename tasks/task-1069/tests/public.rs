use task_1069::*;

#[test]
fn public_examples() {
    assert_eq!(longest_field_with_last_tie_slash(&"path/1/12"), 2);
    assert_eq!(
        longest_field_with_last_tie_slash(&String::from("path/123")),
        3
    );
}
