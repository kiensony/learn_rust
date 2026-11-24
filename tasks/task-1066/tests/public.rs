use task_1066::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_slash(&"path/1/12"), 2);
    assert_eq!(
        remainder_after_first_mark_slash(&String::from("path/123")),
        0
    );
}
