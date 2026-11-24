use task_381::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_pipe(&String::from("first\n123")), 3);
    assert_eq!(
        closing_marks_removed_pipe(&String::from("header\n猫|123")),
        7
    );
}
