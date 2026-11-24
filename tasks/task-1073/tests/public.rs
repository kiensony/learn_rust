use task_1073::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_pipe(&"first\n123"), 3);
    assert_eq!(
        leading_marks_removed_pipe(&String::from("header\n猫|123")),
        7
    );
}
