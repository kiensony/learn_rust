use task_514::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_colon("hi:kit"), 2);
    assert_eq!(prefix_before_final_mark_colon(""), 0);
}
