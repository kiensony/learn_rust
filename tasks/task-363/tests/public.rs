use task_363::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_colon(&String::from("hi:kit")), 3);
    assert_eq!(remainder_after_first_mark_colon(&String::from(":é:a")), 4);
}
