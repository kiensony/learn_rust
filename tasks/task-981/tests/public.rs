use task_981::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_pipe(vec!["first\n123", "header\n猫|123"]),
        vec![3usize, 3usize]
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe(vec!["header\n猫|123", "hi|kit"]),
        vec![3usize, 2usize]
    );
}
