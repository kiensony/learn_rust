use task_976::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_pipe(vec!["first\n123", "header\n猫|123"]),
        vec![3usize, 7usize]
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe(vec!["header\n猫|123", "hi|kit"]),
        vec![7usize, 6usize]
    );
}
