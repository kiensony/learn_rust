use task_380::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_pipe(&String::from("first\n123")), 3);
    assert_eq!(
        leading_marks_removed_pipe(&String::from("header\n猫|123")),
        7
    );
}
