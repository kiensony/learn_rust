use task_977::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_pipe(vec!["first\n123", "header\n猫|123"]),
        vec![0usize, 3usize]
    );
    assert_eq!(
        remainder_after_first_mark_pipe(vec!["header\n猫|123", "hi|kit"]),
        vec![3usize, 3usize]
    );
}
