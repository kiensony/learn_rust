use task_1106::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_colon(&"hi:kit"), 3);
    assert_eq!(remainder_after_first_mark_colon(&String::from(":é:a")), 4);
}
