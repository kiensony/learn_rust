use task_1008::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_colon(["hi:kit", ":é:a"]), [2, 3]);
    assert_eq!(
        prefix_before_final_mark_colon([String::from(":é:a"), String::from("")]),
        [3, 0]
    );
}
