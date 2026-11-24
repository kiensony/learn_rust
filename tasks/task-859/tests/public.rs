use task_859::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_colon("hi:kit"), 4);
    assert_eq!(prefix_before_final_mark_colon(":é:a"), 5);
}
