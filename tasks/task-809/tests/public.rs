use task_809::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_colon("hi:kit"), Ok(2));
    assert_eq!(prefix_before_final_mark_colon(":é:a"), Ok(3));
}
