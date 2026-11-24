use task_371::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_slash(&String::from("path/1/12")), 4);
    assert_eq!(closing_marks_removed_slash(&String::from("path/123")), 3);
}
