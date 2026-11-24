use task_464::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_colon(("hi:kit", "x")), 2);
    assert_eq!(prefix_before_final_mark_colon((":é:a", "z:w")), 3);
}
