use task_980::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_pipe(vec!["first\n123", "header\n猫|123"]),
        vec![3usize, 3usize]
    );
    assert_eq!(longest_field_with_last_tie_pipe(vec![]), vec![]);
}
