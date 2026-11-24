use task_972::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_field_pipe(vec!["first\n123", "header\n猫|123"]),
        vec![3usize, 3usize]
    );
    assert_eq!(
        leading_field_pipe(vec!["header\n猫|123", "hi|kit"]),
        vec![3usize, 2usize]
    );
}
