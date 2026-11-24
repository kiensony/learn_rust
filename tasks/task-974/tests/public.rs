use task_974::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_marks_removed_pipe(vec!["first\n123", "header\n猫|123"]),
        vec![3usize, 7usize]
    );
    assert_eq!(
        leading_marks_removed_pipe(vec!["header\n猫|123", "hi|kit"]),
        vec![7usize, 6usize]
    );
}
